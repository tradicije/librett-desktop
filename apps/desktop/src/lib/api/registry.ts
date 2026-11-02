import { invoke } from '@tauri-apps/api/core';

export interface RegistryFields {name:string;club:string;birth_year:number|null;country:string}

export interface RegistryRemotePlayer {id:string;revision:string;slug:string;display_name:string;birth_year:number|null;country:string|null;region:string|null;given_name:string|null;family_name:string|null;biography:string|null;photo_id:string|null}

export interface RegistryDecision {remote_id:string;local_id:string|null;birth_year:number|null;name:string|null;club:string|null;use_registry:string[];skip:boolean;photo:string|null}

export interface RegistryPreviewRow {remote_id:string;local_id:string;revision:string;source_name:string;current:RegistryFields|null;proposed:RegistryFields;conflicts:string[];overrides:Record<string,boolean>;skip:boolean;valid:boolean;expected_hash:string|null;remote:RegistryRemotePlayer;remote_clubs:string[];photo:string|null}

export interface RegistryPreview {id:string;registry_id:string;registry_name:string;checkpoint:string;source_url:string|null;expected_checkpoint:string|null;expected_source_hash:string|null;policy:Record<string,unknown>;rows:RegistryPreviewRow[];withdrawals:number}

export interface RegistrySource {registry_id:string;name:string;checkpoint:string;source_url:string|null}

export const previewRegistry=(payload:string|null,sourceUrl:string|null,decisions:RegistryDecision[])=>invoke<RegistryPreview>('preview_registry',{payload,sourceUrl,decisions});

export const refineRegistry=(id:string,decisions:RegistryDecision[])=>invoke<RegistryPreview>('refine_registry',{id,decisions});

export const confirmRegistryImport=(id:string)=>invoke<{players:number;withdrawn:number;checkpoint:string}>('confirm_registry_import',{id});

export const registrySources=()=>invoke<RegistrySource[]>('registry_sources');

export const registryPreviews=()=>invoke<RegistryPreview[]>('registry_previews');

export const cancelRegistryPreview=(id:string)=>invoke<void>('cancel_registry_preview',{id});

export const downloadRegistryPhoto=(id:string,remoteId:string)=>invoke<[string,string]>('download_registry_photo',{id,remoteId});
