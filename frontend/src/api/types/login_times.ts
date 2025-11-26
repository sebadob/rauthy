export interface Argon2ParamsResponse {
	m_cost: number;
	t_cost: number;
	p_cost: number;
}

export interface LoginTimeResponse {
	argon2_params: Argon2ParamsResponse;
	login_time: number;
	num_cpus: number;
}
