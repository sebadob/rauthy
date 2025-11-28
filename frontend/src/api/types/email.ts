export type EmailContentType = 'text' | 'markdown' | 'html';
export type EmailJobFilterType = 'none' | 'in_group' | 'not_in_group' | 'has_role' | 'has_not_role';
export type EmailJobStatus = 'open' | 'finished' | 'canceled';

export interface EmailJobRequest {
    /// Validation: Unix TS in the future
    scheduled?: number;
    filter_type: EmailJobFilterType;
    /// Validation: `^[a-zA-Z0-9-_/,:*\s]{2,64}$`
    filter_value?: string;
    content_type: EmailContentType;
    /// Validation: max length 1024
    subject: string;
    /// Validation: max length 32768
    body: string;
}

export interface EmailJobResponse {
    id: number;
    scheduled?: number;
    status: EmailJobStatus;
    updated: number;
    last_user_ts: number;
    filter_type: EmailJobFilterType;
    filter_value?: string;
    content_type: EmailContentType;
    subject: string;
    body: string;
}
