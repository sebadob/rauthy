export type JwkKeyPairAlg = 'RS256' | 'RS384' | 'RS512' | 'EdDSA';
export type JwkKeyPairType = 'RSA' | 'OKP';

export interface JWKSPublicKeyCerts {
    kty: JwkKeyPairType;
    use: string;
    alg: JwkKeyPairAlg;
    // Ed25519
    crv?: string;
    kid?: string;
    // RSA
    n?: string;
    // RSA
    e?: string;
    // OCT
    x?: string;
}

export interface JWKSCerts {
    keys: JWKSPublicKeyCerts[];
}
