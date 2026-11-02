import { invoke } from '@tauri-apps/api/core';

export const openDataFolder=(kind:'backups'|'exports')=>invoke<void>('open_data_folder',{kind});

export const openOfficialLink=(destination:'website'|'source')=>invoke<void>('open_official_link',{destination});
