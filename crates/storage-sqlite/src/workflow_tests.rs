use super::*;
use librett_application::{
    self as app, CategoryRulesRepository, CompletionRequest, PlayerCashRepository,
    PlayerRepository, SaveFillersRequest, SaveMatchRequest, ScheduleAction, ScheduleRequest,
};
use librett_domain::{
    CategoryRules, DrawMode, DrawSettings, FillerChoice, KnockoutFilling, MatchOutcome,
    MatchResult, PlayerProfile, SetScore, ThirdPlaceRule,
};
use std::collections::HashMap;

fn repository() -> SqliteTournamentRepository {
    SqliteTournamentRepository::initialize(Connection::open_in_memory().unwrap()).unwrap()
}
fn category(
    repo: &mut SqliteTournamentRepository,
    tournament: Uuid,
    name: &str,
    format: CompetitionFormat,
    rules: CategoryRules,
    players: &[Uuid],
) -> Uuid {
    let t = app::add_category_with_fee(repo, tournament, name, Discipline::Singles, format, 50_000)
        .unwrap();
    let id = t.categories.last().unwrap().id;
    repo.save_category_rules(tournament, id, &rules, 0).unwrap();
    app::register_entries(
        repo,
        tournament,
        id,
        players.iter().map(|id| vec![*id]).collect(),
    )
    .unwrap();
    for player in players {
        app::set_player_attendance(repo, tournament, *player, true).unwrap();
    }
    let draft = app::preview_category_draw(
        repo,
        tournament,
        id,
        DrawMode::Automatic,
        DrawSettings {
            group_count: rules.group_count,
            qualifiers_per_group: rules.qualifiers_per_group,
        },
        vec![],
    )
    .unwrap();
    app::save_category_draw(repo, tournament, draft, 0, false).unwrap();
    id
}
fn players(repo: &mut SqliteTournamentRepository, n: usize) -> Vec<Uuid> {
    (0..n)
        .map(|i| {
            app::save_player_profile(
                repo,
                None,
                &format!("Igrač Đorđe {i}"),
                "Bubušinac",
                PlayerProfile {
                    birth_year: Some(2000),
                    ..Default::default()
                },
            )
            .unwrap()
            .id
        })
        .collect()
}
fn score(
    repo: &mut SqliteTournamentRepository,
    tournament: Uuid,
    category: Uuid,
    item: librett_domain::ScheduledMatch,
    side: bool,
    confirm: bool,
) -> Result<librett_domain::ScheduledMatch, ApplicationError> {
    let configuration = repo.find_category_rules(category)?;
    let draw = app::get_category_draw(repo, tournament, category)?.unwrap();
    let first = item.first.unwrap();
    let second = item.second.unwrap();
    repo.save_match_result(SaveMatchRequest {
        allow_unconfirmed_start: false,
        request_id: Uuid::new_v4(),
        tournament_id: tournament,
        category_id: category,
        draw_id: draw.id,
        key: item.key,
        expected_revision: item.revision,
        rules_revision: configuration.revision,
        result: MatchResult {
            first,
            second,
            winner: if side { second } else { first },
            outcome: MatchOutcome::Played,
            sets: (0..configuration.rules.best_of / 2 + 1)
                .map(|_| {
                    if side {
                        SetScore {
                            first: 0,
                            second: 11,
                        }
                    } else {
                        SetScore {
                            first: 11,
                            second: 0,
                        }
                    }
                })
                .collect(),
            rules: configuration.rules,
        },
        invalidate_downstream: confirm,
    })
}
fn finish_knockout(repo: &mut SqliteTournamentRepository, tournament: Uuid, category: Uuid) {
    for _ in 0..20 {
        let state = repo.competition_state(tournament, category).unwrap();
        let ready = state
            .matches
            .into_iter()
            .filter(|m| m.first.is_some() && m.second.is_some() && !m.bye && m.result.is_none())
            .collect::<Vec<_>>();
        if ready.is_empty() {
            return;
        }
        for item in ready {
            score(repo, tournament, category, item, false, false).unwrap();
        }
    }
    panic!("knockout did not finish");
}
fn complete(
    repo: &mut SqliteTournamentRepository,
    tournament: Uuid,
    category: Option<Uuid>,
    value: bool,
) -> Result<app::TournamentProgress, ApplicationError> {
    let (revision, version) = if let Some(id) = category {
        let data = repo.category_results(tournament, id)?;
        (data.completion.revision, data.version)
    } else {
        let data = repo.tournament_progress(tournament)?;
        (data.completion.revision, data.version)
    };
    repo.change_completion(CompletionRequest {
        request_id: Uuid::new_v4(),
        tournament_id: tournament,
        category_id: category,
        expected_revision: revision,
        expected_version: version,
        complete: value,
    })
}
fn finish_groups(repo: &mut SqliteTournamentRepository, tournament: Uuid, category: Uuid) {
    let draw = app::get_category_draw(repo, tournament, category)
        .unwrap()
        .unwrap();
    let priority: HashMap<_, _> = draw
        .participants
        .iter()
        .enumerate()
        .map(|(i, e)| (e.id, i))
        .collect();
    for (group, section) in draw.sections.iter().enumerate() {
        let n = section.iter().flatten().count();
        for round in 0..n + n % 2 - 1 {
            let page = repo
                .match_page(tournament, category, group, round, 0, false)
                .unwrap();
            for item in page.matches {
                let side = priority[&item.second.unwrap()] < priority[&item.first.unwrap()];
                score(repo, tournament, category, item, side, false).unwrap();
            }
        }
    }
}
#[test]
fn odd_draw_scores_completion_locks_reopen_and_history() {
    let mut repo = repository();
    let tournament = app::create_tournament(&mut repo, "Bubušinac 2027")
        .unwrap()
        .id;
    let entrants = players(&mut repo, 5);
    let id = category(
        &mut repo,
        tournament,
        "Singl",
        CompetitionFormat::Knockout,
        CategoryRules::default(),
        &entrants,
    );
    assert_eq!(
        complete(&mut repo, tournament, Some(id), true).unwrap_err(),
        ApplicationError::CompetitionIncomplete
    );
    finish_knockout(&mut repo, tournament, id);
    let results = repo.category_results(tournament, id).unwrap();
    assert!(results.ready);
    assert_eq!(results.knockout_total, 4);
    assert_eq!(results.placements.len(), 5);
    let request = CompletionRequest {
        request_id: Uuid::new_v4(),
        tournament_id: tournament,
        category_id: Some(id),
        expected_revision: results.completion.revision,
        expected_version: results.version,
        complete: true,
    };
    repo.change_completion(request.clone()).unwrap();
    assert!(repo.change_completion(request).is_ok());
    assert_eq!(
        repo.save_category_rules(
            tournament,
            id,
            &CategoryRules {
                best_of: 7,
                ..Default::default()
            },
            1
        )
        .unwrap_err(),
        ApplicationError::CompetitionClosed
    );
    complete(&mut repo, tournament, None, true).unwrap();
    assert_eq!(
        complete(&mut repo, tournament, Some(id), false).unwrap_err(),
        ApplicationError::CompetitionClosed
    );
    complete(&mut repo, tournament, None, false).unwrap();
    assert!(repo
        .category_results(tournament, id)
        .unwrap()
        .completion
        .completed_at
        .is_some());
    complete(&mut repo, tournament, Some(id), false).unwrap();
    assert_eq!(
        repo.connection
            .query_row("SELECT count(*) FROM completion_history", [], |r| r
                .get::<_, i64>(0))
            .unwrap(),
        4
    );
    assert!(!repo.find(tournament).unwrap().completed);
}
#[test]
fn bronze_required_and_semifinal_correction_invalidates_dependents() {
    let mut repo = repository();
    let tournament = app::create_tournament(&mut repo, "Bronza").unwrap().id;
    let entrants = players(&mut repo, 4);
    let id = category(
        &mut repo,
        tournament,
        "Bronza",
        CompetitionFormat::Knockout,
        CategoryRules {
            third_place: ThirdPlaceRule::BronzeMatch,
            ..Default::default()
        },
        &entrants,
    );
    for item in repo
        .competition_state(tournament, id)
        .unwrap()
        .matches
        .into_iter()
        .filter(|m| m.round == 0)
    {
        score(&mut repo, tournament, id, item, false, false).unwrap();
    }
    let state = repo.competition_state(tournament, id).unwrap();
    let final_match = state
        .matches
        .iter()
        .find(|m| m.round == 1 && m.position == 0)
        .unwrap()
        .clone();
    let bronze = state
        .matches
        .iter()
        .find(|m| m.round == 1 && m.position == 1)
        .unwrap()
        .clone();
    score(&mut repo, tournament, id, final_match, false, false).unwrap();
    assert!(!repo.category_results(tournament, id).unwrap().ready);
    score(&mut repo, tournament, id, bronze, false, false).unwrap();
    let results = repo.category_results(tournament, id).unwrap();
    assert!(results.ready);
    assert_eq!(results.knockout_total, 4);
    assert_eq!(
        results
            .placements
            .iter()
            .map(|p| p.place)
            .collect::<Vec<_>>(),
        vec![1, 2, 3, 4]
    );
    let semifinal = repo
        .competition_state(tournament, id)
        .unwrap()
        .matches
        .into_iter()
        .find(|m| m.round == 0 && m.position == 0)
        .unwrap();
    assert_eq!(
        score(&mut repo, tournament, id, semifinal.clone(), true, false).unwrap_err(),
        ApplicationError::ResultImpact
    );
    score(&mut repo, tournament, id, semifinal, true, true).unwrap();
    let state = repo.competition_state(tournament, id).unwrap();
    assert!(state
        .matches
        .iter()
        .filter(|m| m.round == 1)
        .all(|m| m.result.is_none()));
}
#[test]
fn champion_semifinalist_is_third_without_an_extra_match() {
    let mut repo = repository();
    let tournament = app::create_tournament(&mut repo, "Treće").unwrap().id;
    let entrants = players(&mut repo, 4);
    let id = category(
        &mut repo,
        tournament,
        "Treće",
        CompetitionFormat::Knockout,
        CategoryRules {
            third_place: ThirdPlaceRule::ChampionSemifinalist,
            ..Default::default()
        },
        &entrants,
    );
    finish_knockout(&mut repo, tournament, id);
    let state = repo.competition_state(tournament, id).unwrap();
    assert_eq!(state.matches.len(), 3);
    let winner = state
        .matches
        .last()
        .unwrap()
        .result
        .as_ref()
        .unwrap()
        .winner;
    let semi = state
        .matches
        .iter()
        .find(|m| m.round == 0 && m.result.as_ref().unwrap().winner == winner)
        .unwrap()
        .result
        .as_ref()
        .unwrap();
    let third = if semi.winner == semi.first {
        semi.second
    } else {
        semi.first
    };
    let results = repo.category_results(tournament, id).unwrap();
    assert!(results.ready);
    assert_eq!(
        results
            .placements
            .iter()
            .find(|p| p.place == 3)
            .unwrap()
            .entry_id,
        third
    );
}
#[test]
fn automatic_lucky_loser_full_tournament() {
    let mut repo = repository();
    let tournament = app::create_tournament(&mut repo, "LL").unwrap().id;
    let entrants = players(&mut repo, 6);
    let id = category(
        &mut repo,
        tournament,
        "LL",
        CompetitionFormat::GroupsKnockout,
        CategoryRules {
            group_count: 1,
            qualifiers_per_group: 3,
            knockout_filling: KnockoutFilling::LuckyLoser,
            third_place: ThirdPlaceRule::BronzeMatch,
            ..Default::default()
        },
        &entrants,
    );
    finish_groups(&mut repo, tournament, id);
    let state = repo.competition_state(tournament, id).unwrap();
    assert!(state.groups.iter().all(|g| g.complete && g.resolved));
    assert_eq!(
        state
            .slots
            .iter()
            .filter(|s| s.lucky_loser && s.entry_id.is_some())
            .count(),
        1
    );
    finish_knockout(&mut repo, tournament, id);
    let results = repo.category_results(tournament, id).unwrap();
    assert!(results.ready);
    assert_eq!(results.placements.len(), 6);
    complete(&mut repo, tournament, Some(id), true).unwrap();
    complete(&mut repo, tournament, None, true).unwrap();
}
#[test]
fn manual_lucky_loser_waits_for_choice_and_can_mix_bye() {
    let mut repo = repository();
    let tournament = app::create_tournament(&mut repo, "Ručni LL").unwrap().id;
    let entrants = players(&mut repo, 6);
    let id = category(
        &mut repo,
        tournament,
        "Ručni LL",
        CompetitionFormat::GroupsKnockout,
        CategoryRules {
            group_count: 1,
            qualifiers_per_group: 3,
            knockout_filling: KnockoutFilling::LuckyLoserManual,
            ..Default::default()
        },
        &entrants,
    );
    finish_groups(&mut repo, tournament, id);
    let state = repo.competition_state(tournament, id).unwrap();
    let index = state.slots.iter().position(|s| s.lucky_loser).unwrap();
    assert!(state.slots[index].entry_id.is_none() && !state.slots[index].bye);
    let request = SaveFillersRequest {
        request_id: Uuid::new_v4(),
        tournament_id: tournament,
        category_id: id,
        draw_id: state.draw_id.unwrap(),
        expected_revision: state.filler_revision,
        expected_match_version: state.match_version,
        rules_revision: state.rules_revision,
        order_revisions: state.order_revisions,
        result_versions: state.result_versions,
        fillers: HashMap::from([(index, FillerChoice::Bye)]),
        invalidate_downstream: false,
    };
    let result = repo.save_knockout_fillers(request.clone()).unwrap();
    assert!(result.slots[index].bye);
    assert!(repo.save_knockout_fillers(request).is_ok());
    finish_knockout(&mut repo, tournament, id);
    assert!(repo.category_results(tournament, id).unwrap().ready);
}
#[test]
fn tables_check_cross_category_players_occupied_tables_and_retries() {
    let mut repo = repository();
    let tournament = app::create_tournament(&mut repo, "Stolovi").unwrap().id;
    let entrants = players(&mut repo, 2);
    let a = category(
        &mut repo,
        tournament,
        "A",
        CompetitionFormat::Knockout,
        CategoryRules::default(),
        &entrants,
    );
    let b = category(
        &mut repo,
        tournament,
        "B",
        CompetitionFormat::Knockout,
        CategoryRules::default(),
        &entrants,
    );
    let state = repo.schedule(tournament).unwrap();
    let first = state.waiting.iter().find(|m| m.category_id == a).unwrap();
    let request = ScheduleRequest {
        request_id: Uuid::new_v4(),
        tournament_id: tournament,
        expected_version: state.version.clone(),
        action: ScheduleAction::Assign {
            draw_id: first.draw_id,
            key: first.key.clone(),
            table: 1,
        },
    };
    let state = repo.change_schedule(request.clone()).unwrap();
    assert!(repo.change_schedule(request).is_ok());
    let second = state.waiting.iter().find(|m| m.category_id == b).unwrap();
    let request = ScheduleRequest {
        request_id: Uuid::new_v4(),
        tournament_id: tournament,
        expected_version: state.version,
        action: ScheduleAction::Assign {
            draw_id: second.draw_id,
            key: second.key.clone(),
            table: 2,
        },
    };
    assert_eq!(
        repo.change_schedule(request).unwrap_err(),
        ApplicationError::PlayerBusy
    );
    let state = repo.schedule(tournament).unwrap();
    let first = &state.assignments[0].scheduled;
    let table_move = ScheduleRequest {
        request_id: Uuid::new_v4(),
        tournament_id: tournament,
        expected_version: state.version,
        action: ScheduleAction::Assign {
            draw_id: first.draw_id,
            key: first.key.clone(),
            table: 2,
        },
    };
    let state = repo.change_schedule(table_move).unwrap();
    assert_eq!(state.assignments[0].table, 2);
    let first_match = repo
        .competition_state(tournament, a)
        .unwrap()
        .matches
        .remove(0);
    score(&mut repo, tournament, a, first_match, false, false).unwrap();
    let state = repo.schedule(tournament).unwrap();
    assert!(state.assignments.is_empty());
    let second = &state.waiting[0];
    let state = repo
        .change_schedule(ScheduleRequest {
            request_id: Uuid::new_v4(),
            tournament_id: tournament,
            expected_version: state.version.clone(),
            action: ScheduleAction::Assign {
                draw_id: second.draw_id,
                key: second.key.clone(),
                table: 2,
            },
        })
        .unwrap();
    assert_eq!(state.assignments.len(), 1);
}
#[test]
fn backup_restore_validates_before_replacement_and_preserves_safety_copy() {
    let directory = std::env::temp_dir().join(format!("librett-backup-test-{}", Uuid::new_v4()));
    let mut repo = repository();
    let tournament = app::create_tournament(&mut repo, "Đorđe i Željko")
        .unwrap()
        .id;
    let entrants = players(&mut repo, 4);
    let id = category(
        &mut repo,
        tournament,
        "Žreb",
        CompetitionFormat::Knockout,
        CategoryRules::default(),
        &entrants,
    );
    finish_knockout(&mut repo, tournament, id);
    complete(&mut repo, tournament, Some(id), true).unwrap();
    let backup = repo.create_backup(&directory, "manual").unwrap();
    app::create_tournament(&mut repo, "Kasniji turnir").unwrap();
    assert_eq!(repo.list().unwrap().len(), 2);
    assert_eq!(
        repo.import_backup(&directory, b"not sqlite").unwrap_err(),
        ApplicationError::InvalidBackup
    );
    assert_eq!(repo.list().unwrap().len(), 2);
    assert!(SqliteTournamentRepository::backup_path(&directory, "../librett.sqlite").is_err());
    repo.restore_backup(&directory, &backup.name).unwrap();
    assert_eq!(repo.list().unwrap().len(), 1);
    assert_eq!(repo.list().unwrap()[0].name, "Đorđe i Željko");
    assert!(repo
        .category_results(tournament, id)
        .unwrap()
        .completion
        .completed_at
        .is_some());
    assert!(!repo.cash_ledger(tournament).unwrap().records.is_empty());
    let safety = SqliteTournamentRepository::backups(&directory)
        .unwrap()
        .into_iter()
        .find(|b| b.name.starts_with("librett-before-restore-"))
        .unwrap();
    repo.restore_backup(&directory, &safety.name).unwrap();
    assert_eq!(repo.list().unwrap().len(), 2);
    repo.automatic_backup(&directory).unwrap();
    repo.automatic_backup(&directory).unwrap();
    assert_eq!(
        SqliteTournamentRepository::backups(&directory)
            .unwrap()
            .iter()
            .filter(|b| b.name.starts_with("librett-auto-"))
            .count(),
        1
    );
    std::fs::remove_dir_all(directory).unwrap();
}

#[test]
fn tournament_trash_preserves_results_cash_history_and_backup_restore() {
    let directory = std::env::temp_dir().join(format!("librett-trash-test-{}", Uuid::new_v4()));
    let path = directory.join("live.sqlite");
    std::fs::create_dir_all(&directory).unwrap();
    let mut repo = SqliteTournamentRepository::open(&path).unwrap();
    let tournament = app::create_tournament(&mut repo, "Završeni kup")
        .unwrap()
        .id;
    let other = app::create_tournament(&mut repo, "Aktivni kup").unwrap();
    let entrants = players(&mut repo, 4);
    let id = category(
        &mut repo,
        tournament,
        "Singl",
        CompetitionFormat::Knockout,
        CategoryRules::default(),
        &entrants,
    );
    let entries = repo.list_entries(id).unwrap();
    let payment = librett_domain::CashRecord {
        id: Uuid::new_v4(),
        entry_id: entries[0].id,
        kind: librett_domain::CashKind::Payment,
        amount_minor: 50_000,
        note: String::new(),
        created_at: String::new(),
    };
    app::record_cash(&mut repo, tournament, payment).unwrap();
    finish_knockout(&mut repo, tournament, id);
    complete(&mut repo, tournament, Some(id), true).unwrap();
    complete(&mut repo, tournament, None, true).unwrap();
    let original = repo.find(tournament).unwrap();
    let results = serde_json::to_value(repo.category_results(tournament, id).unwrap()).unwrap();
    let cash = repo.cash_ledger(tournament).unwrap();
    let history: i64 = repo
        .connection
        .query_row("SELECT count(*) FROM completion_history", [], |r| r.get(0))
        .unwrap();
    repo.trash_tournament(tournament).unwrap();
    repo.trash_tournament(tournament).unwrap(); // Retrying cannot duplicate or lose data.
    assert_eq!(repo.list().unwrap(), vec![other.clone()]);
    assert_eq!(repo.find(tournament), Err(ApplicationError::NotFound));
    assert_eq!(
        repo.list_trashed_tournaments().unwrap(),
        vec![original.clone()]
    );
    assert!(repo
        .update_tournament_details(tournament, "Changed", None, &original.name, None)
        .is_err());
    // Cash remains editable on completed tournaments, but never while in trash.
    assert!(repo.connection.execute("INSERT INTO cash_records(id,entry_id,kind,amount_minor,note) VALUES(?1,?2,'payment',1,'')", params![Uuid::new_v4().to_string(),entries[0].id.to_string()]).is_err());
    assert!(repo
        .connection
        .execute(
            "UPDATE tournaments SET name='Changed' WHERE id=?1",
            [tournament.to_string()]
        )
        .is_err());
    let backup = repo.create_backup(&directory, "manual").unwrap();
    drop(repo);
    let mut repo = SqliteTournamentRepository::open(&path).unwrap();
    assert_eq!(
        repo.list_trashed_tournaments().unwrap(),
        vec![original.clone()]
    );
    assert_eq!(repo.restore_tournament(tournament).unwrap(), original);
    assert_eq!(repo.restore_tournament(tournament).unwrap(), original);
    assert_eq!(
        serde_json::to_value(repo.category_results(tournament, id).unwrap()).unwrap(),
        results
    );
    assert_eq!(repo.cash_ledger(tournament).unwrap(), cash);
    assert_eq!(
        repo.connection
            .query_row("SELECT count(*) FROM completion_history", [], |r| r
                .get::<_, i64>(0))
            .unwrap(),
        history
    );
    assert_eq!(repo.list_entries(id).unwrap(), entries);
    assert_eq!(repo.find(other.id).unwrap(), other);
    assert_eq!(repo.list_players().unwrap().len(), entrants.len());
    repo.restore_backup(&directory, &backup.name).unwrap();
    assert_eq!(
        repo.list_trashed_tournaments().unwrap(),
        vec![original.clone()]
    );
    assert_eq!(repo.restore_tournament(tournament).unwrap(), original);
    assert_eq!(
        serde_json::to_value(repo.category_results(tournament, id).unwrap()).unwrap(),
        results
    );
    drop(repo);
    std::fs::remove_dir_all(directory).unwrap();
}

#[test]
fn trashed_open_tournament_rejects_stale_sporting_writes_and_unknown_ids() {
    let mut repo = repository();
    let tournament = app::create_tournament(&mut repo, "Otvoreni kup")
        .unwrap()
        .id;
    let entrants = players(&mut repo, 2);
    let id = category(
        &mut repo,
        tournament,
        "Singl",
        CompetitionFormat::Knockout,
        CategoryRules::default(),
        &entrants,
    );
    repo.trash_tournament(tournament).unwrap();
    assert_eq!(
        app::add_category(
            &mut repo,
            tournament,
            "Dubl",
            Discipline::Doubles,
            CompetitionFormat::Knockout
        ),
        Err(ApplicationError::NotFound)
    );
    assert_eq!(
        app::set_player_attendance(&mut repo, tournament, entrants[0], true),
        Err(ApplicationError::NotFound)
    );
    assert!(repo
        .connection
        .execute(
            "UPDATE categories SET name='Stale' WHERE id=?1",
            [id.to_string()]
        )
        .is_err());
    assert!(repo
        .connection
        .execute(
            "UPDATE entries SET status='withdrawn' WHERE category_id=?1",
            [id.to_string()]
        )
        .is_err());
    assert_eq!(
        repo.trash_tournament(Uuid::new_v4()),
        Err(ApplicationError::NotFound)
    );
    assert_eq!(
        repo.restore_tournament(Uuid::new_v4()),
        Err(ApplicationError::NotFound)
    );
    repo.restore_tournament(tournament).unwrap();
    app::set_player_attendance(&mut repo, tournament, entrants[0], true).unwrap();
    finish_knockout(&mut repo, tournament, id);
    assert!(repo.category_results(tournament, id).unwrap().ready);
}

#[test]
fn first_result_checks_arrivals_and_explicit_override_does_not_enable_payment() {
    let mut repo = repository();
    let tournament = app::create_tournament(&mut repo, "Prepared draw")
        .unwrap()
        .id;
    let entrants = players(&mut repo, 2);
    let id = category(
        &mut repo,
        tournament,
        "Singles",
        CompetitionFormat::Knockout,
        CategoryRules::default(),
        &entrants,
    );
    for p in &entrants {
        app::set_player_attendance(&mut repo, tournament, *p, false).unwrap();
    }
    let schedule = repo.schedule(tournament).unwrap();
    let ready = schedule.waiting[0].clone();
    let queued = repo
        .change_schedule(ScheduleRequest {
            request_id: Uuid::new_v4(),
            tournament_id: tournament,
            expected_version: schedule.version,
            action: ScheduleAction::Assign {
                draw_id: ready.draw_id,
                key: ready.key,
                table: 1,
            },
        })
        .unwrap();
    assert_eq!(
        repo.change_schedule(ScheduleRequest {
            request_id: Uuid::new_v4(),
            tournament_id: tournament,
            expected_version: queued.version,
            action: ScheduleAction::Start { table: 1 },
        })
        .unwrap_err(),
        ApplicationError::AttendanceRequired
    );
    assert_eq!(
        repo.schedule(tournament).unwrap().assignments[0].status,
        "queued"
    );
    let state = repo.competition_state(tournament, id).unwrap();
    let item = state
        .matches
        .iter()
        .find(|m| m.first.is_some() && m.second.is_some())
        .unwrap()
        .clone();
    assert_eq!(
        score(&mut repo, tournament, id, item.clone(), false, false).unwrap_err(),
        ApplicationError::AttendanceRequired
    );
    assert!(!repo.registration_started(tournament, id).unwrap());
    assert_eq!(
        repo.competition_state(tournament, id).unwrap().matches[0].result,
        None
    );
    let entries = repo.list_entries(id).unwrap();
    repo.save_match_result(SaveMatchRequest {
        request_id: Uuid::new_v4(),
        tournament_id: tournament,
        category_id: id,
        draw_id: state.draw_id.unwrap(),
        rules_revision: state.rules_revision,
        key: item.key,
        expected_revision: item.revision,
        invalidate_downstream: false,
        allow_unconfirmed_start: true,
        result: MatchResult {
            first: item.first.unwrap(),
            second: item.second.unwrap(),
            winner: item.first.unwrap(),
            outcome: MatchOutcome::Walkover,
            sets: vec![],
            rules: repo.find_category_rules(id).unwrap().rules,
        },
    })
    .unwrap();
    assert!(repo.registration_started(tournament, id).unwrap());
    assert!(repo
        .list_entries(id)
        .unwrap()
        .iter()
        .all(|e| e.members.iter().all(|m| !m.checked_in)));
    assert_eq!(
        repo.settle_player_cash(
            Uuid::new_v4(),
            tournament,
            entrants[0],
            vec![
                entries
                    .iter()
                    .find(|e| e.members[0].id == entrants[0])
                    .unwrap()
                    .id
            ],
            true
        ),
        Err(ApplicationError::AttendanceRequired)
    );
    let results = serde_json::to_value(repo.category_results(tournament, id).unwrap()).unwrap();
    assert_eq!(
        app::set_entry_status(
            &mut repo,
            tournament,
            entries[0].id,
            librett_domain::EntryStatus::Withdrawn
        ),
        Err(ApplicationError::CompetitionStarted)
    );
    let late = players(&mut repo, 1)[0];
    assert_eq!(
        app::register_entry(&mut repo, tournament, id, vec![late]),
        Err(ApplicationError::CompetitionStarted)
    );
    assert_eq!(
        serde_json::to_value(repo.category_results(tournament, id).unwrap()).unwrap(),
        results
    );
}
