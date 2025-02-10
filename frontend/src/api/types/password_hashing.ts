export interface PasswordHashTimesRequest {
    /// Validation: min 500
    target_time: number,
    /// Validation: min 32768
    m_cost?: number,
    /// Validation: min 2
    p_cost?: number,
}

export interface PasswordHashTime {
    alg: string,
    m_cost: number,
    t_cost: number,
    p_cost: number,
    time_taken: number,
}

export interface PasswordHashTimes {
    results: PasswordHashTime[],
}