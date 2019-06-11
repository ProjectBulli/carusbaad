[DEBUG] - No cached session for DNSNameRef("example.com")
[DEBUG] - Not resuming any session
[TRACE] - Sending ClientHello Message {
    typ: Handshake,
    version: TLSv1_0,
    payload: Handshake(
        HandshakeMessagePayload {
            typ: ClientHello,
            payload: ClientHello(
                ClientHelloPayload {
                    client_version: TLSv1_2,
                    random: Random(
                        [
                            222,
                            109,
                            130,
                            165,
                            54,
                            41,
                            86,
                            39,
                            245,
                            95,
                            252,
                            101,
                            230,
                            140,
                            180,
                            233,
                            244,
                            2,
                            126,
                            104,
                            51,
                            199,
                            228,
                            156,
                            21,
                            90,
                            26,
                            165,
                            158,
                            204,
                            253,
                            205,
                        ],
                    ),
                    session_id: SessionID,
                    cipher_suites: [
                        TLS13_CHACHA20_POLY1305_SHA256,
                        TLS13_AES_256_GCM_SHA384,
                        TLS13_AES_128_GCM_SHA256,
                        TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256,
                        TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256,
                        TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
                        TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,
                        TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,
                        TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256,
                        TLS_EMPTY_RENEGOTIATION_INFO_SCSV,
                    ],
                    compression_methods: [
                        Null,
                    ],
                    extensions: [
                        SupportedVersions(
                            [
                                TLSv1_3,
                                TLSv1_2,
                            ],
                        ),
                        ServerName(
                            [
                                ServerName {
                                    typ: HostName,
                                    payload: HostName(
                                        DNSName(
                                            "example.com",
                                        ),
                                    ),
                                },
                            ],
                        ),
                        ECPointFormats(
                            [
                                Uncompressed,
                            ],
                        ),
                        NamedGroups(
                            [
                                X25519,
                                secp384r1,
                                secp256r1,
                            ],
                        ),
                        SignatureAlgorithms(
                            [
                                ECDSA_NISTP384_SHA384,
                                ECDSA_NISTP256_SHA256,
                                RSA_PSS_SHA512,
                                RSA_PSS_SHA384,
                                RSA_PSS_SHA256,
                                RSA_PKCS1_SHA512,
                                RSA_PKCS1_SHA384,
                                RSA_PKCS1_SHA256,
                            ],
                        ),
                        ExtendedMasterSecretRequest,
                        CertificateStatusRequest(
                            OCSP(
                                OCSPCertificateStatusRequest {
                                    responder_ids: [],
                                    extensions: PayloadU16(
                                        [],
                                    ),
                                },
                            ),
                        ),
                        KeyShare(
                            [
                                KeyShareEntry {
                                    group: X25519,
                                    payload: PayloadU16(
                                        [
                                            101,
                                            149,
                                            59,
                                            110,
                                            123,
                                            128,
                                            172,
                                            31,
                                            80,
                                            229,
                                            107,
                                            213,
                                            0,
                                            13,
                                            9,
                                            153,
                                            136,
                                            219,
                                            139,
                                            111,
                                            39,
                                            191,
                                            44,
                                            10,
                                            26,
                                            126,
                                            75,
                                            164,
                                            50,
                                            61,
                                            24,
                                            56,
                                        ],
                                    ),
                                },
                            ],
                        ),
                        PresharedKeyModes(
                            [
                                PSK_DHE_KE,
                            ],
                        ),
                        SessionTicketRequest,
                    ],
                },
            ),
        },
    ),
}
Error reading Os { code: 107, kind: NotConnected, message: "Transport endpoint is not connected" }
