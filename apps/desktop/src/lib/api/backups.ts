import { invoke } from '@tauri-apps/api/core';

export interface BackupInfo { name:string; size:number; modified:number }

export const listBackups=()=>invoke<BackupInfo[]>('list_backups');

export const createBackup=()=>invoke<BackupInfo>('create_backup');

export const automaticBackup=()=>invoke<void>('automatic_backup');

export const restoreBackup=(name:string)=>invoke<void>('restore_backup',{name});

export const importBackup=(encoded:string)=>invoke<void>('import_backup',{encoded});

export const exportBackup=(name:string)=>invoke<string>('export_backup',{name});
