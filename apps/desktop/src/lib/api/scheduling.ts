import { invoke } from '@tauri-apps/api/core';

export interface ReadyMatch {category_id:string;category_name:string;draw_id:string;key:string;phase:'groups'|'knockout';round:number;first:string;second:string;first_name:string;second_name:string;players:string[]}

export interface TableAssignment {table:number;status:'queued'|'running';scheduled:ReadyMatch;started_at:string|null}

export interface ScheduleState {tournament_id:string;table_count:number;version:string;assignments:TableAssignment[];waiting:ReadyMatch[];truncated:boolean}

export type ScheduleAction={kind:'remove';draw_id:string;key:string}|{kind:'configure';table_count:number}|{kind:'assign';draw_id:string;key:string;table:number}|{kind:'start'|'clear';table:number};

export interface ScheduleRequest {request_id:string;tournament_id:string;expected_version:string;action:ScheduleAction}

export const getSchedule=(tournamentId:string)=>invoke<ScheduleState>('get_schedule',{tournamentId});

export const changeSchedule=(request:ScheduleRequest)=>invoke<ScheduleState>('change_schedule',{request});

export const getMatchTables=(tournamentId:string,drawId:string)=>invoke<Record<string,number>>('get_match_tables',{tournamentId,drawId});
