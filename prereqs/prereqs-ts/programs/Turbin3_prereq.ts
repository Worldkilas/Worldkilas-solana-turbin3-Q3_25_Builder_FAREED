// Type defs for turbin3

export type Turbin3Prereq = {
    address: "TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM",
    // version: "0.1.0",
    name: "Turbin3 Prerequisite",
    description: "A prerequisite for Turbin3, providing essential functionalities.",
    instructions: Array<any>,
    accounts: Array<any>,
    types: Array<any>,
    errors: Array<any>
    metadata?: any,

}

export const IDL: Turbin3Prereq = {
    name: "Turbin3 Prerequisite",
    instructions: [
        {
            name: "initialize",
            discriminator: [175, 175, 109, 31, 13, 152, 155, 237],
            accounts: [
                {
                    name: "user",
                    writable: true,
                    signer: true,
                },
                {
                    name: "account",
                    writable: true,
                    pda: {
                        seeds: [
                            {
                                kind: "const",
                                value: [112, 114, 101, 114, 101, 113, 115]
                            },

                            {
                                kind: "account",
                                path: "user"
                            },
                        ]
                    }
                },
                {
                    name: "system_program",
                    address: "11111111111111111111111111111111",
                },
            ],
            args: [
                {
                    name: "Worldkilas",
                    type: "string",
                }
            ],
        },

        {
        name: "submit_ts",
        discriminator: [137, 241, 199, 223, 125, 33, 85, 217],
        accounts: [
            { name: "user", writable: true,  },
            {
            name: "account",
            writable: true,
            pda: {
                seeds: [
                { kind: "const", value: [112, 114, 101, 114, 101, 113, 115] },
                { kind: "account", path: "user" }
                ]
            }
            },
            { name: "mint", writable: true, signer: true },
            { name: "collection", writable: true },
            {
            name: "authority",
           
            pda: {
                seeds: [
                { kind: "const", value: [99, 111, 108, 108, 101, 99, 116, 105, 111, 110] },
                { kind: "account", path: "collection" }
                ]
            }
            },
            { name: "mpl_core_program", address: "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d" },
            { name: "system_program", address: "11111111111111111111111111111111" }
        ],
        args: []    
            }
        ],
        accounts: [
            {
                name: "ApplicationAccount",
                discriminator: [222, 181, 17, 200, 212, 149, 64, 88]
            }
    ],


    types: [
        {
            name: "ApplicationAccount",
            type: {
                kind: "struct",
                fields: [
                    {
                        name: "user",
                        type: "pubkey"
                    },
                    {
                        name: "bump",
                        type: "u8"
                    },
                    {
                        name: "pre_req_ts",
                        type: "bool",
                    }
                ]
            }
        }
    ],
    errors: [
        {
            code: 6000,
            name: "PreReqTsNotComplete",
            msg: "TS submission not completed."
        },
        {
            code: 6001,
            name: "PreReqTsAlreadyCompleted",
            msg: "TS submission already completed."
        },
    ],
    address: "TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM",
    // version: "0.1.0",
    description: "A prerequisite for Turbin3, providing essential functionalities."
}