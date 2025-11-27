export interface BackupListing {
    name: string;
    // Unix Timestamp
    last_modified: number;
    size?: number;
}

export interface BackupListings {
    local: BackupListing[];
    s3: BackupListing[];
}
