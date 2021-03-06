export type Ehr = {
  "version": "0.1.0",
  "name": "ehr",
  "instructions": [
    {
      "name": "initBloodtestBooking",
      "accounts": [
        {
          "name": "bloodtestReport",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patientProfile",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "technicianProfile",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patientDepositTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patient",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initDoctorProfile",
      "accounts": [
        {
          "name": "doctorProfile",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "doctor",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "doctorName",
          "type": "string"
        },
        {
          "name": "doctorSex",
          "type": "string"
        },
        {
          "name": "doctorDob",
          "type": "string"
        },
        {
          "name": "doctorExperienceMonths",
          "type": "u32"
        },
        {
          "name": "doctorGcFee",
          "type": "u64"
        },
        {
          "name": "doctorLicence",
          "type": "bool"
        },
        {
          "name": "doctorGcExpiryTime",
          "type": "i64"
        }
      ]
    },
    {
      "name": "initGcBooking",
      "accounts": [
        {
          "name": "generalConsultancy",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patientProfile",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "doctorProfile",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patientDepositTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patient",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "patientComments",
          "type": "string"
        }
      ]
    },
    {
      "name": "initPatientProfile",
      "accounts": [
        {
          "name": "patientProfile",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patient",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "sex",
          "type": "string"
        },
        {
          "name": "dob",
          "type": "string"
        }
      ]
    },
    {
      "name": "initTechnicianProfile",
      "accounts": [
        {
          "name": "technicianProfile",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "technician",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "technicianName",
          "type": "string"
        },
        {
          "name": "technicianSex",
          "type": "string"
        },
        {
          "name": "technicianDob",
          "type": "string"
        },
        {
          "name": "technicianExperienceMonths",
          "type": "u32"
        },
        {
          "name": "technicianBloodtestFee",
          "type": "u64"
        },
        {
          "name": "technicianLicence",
          "type": "bool"
        },
        {
          "name": "technicianBloodtestExpiryTime",
          "type": "i64"
        }
      ]
    },
    {
      "name": "updateBloodtestReport",
      "accounts": [
        {
          "name": "bloodtestReport",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "technicianProfile",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "technician",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "whiteBloodCells",
          "type": "u128"
        },
        {
          "name": "redBloodCells",
          "type": "u128"
        },
        {
          "name": "bloodPlatelets",
          "type": "u128"
        },
        {
          "name": "technicianComments",
          "type": "string"
        },
        {
          "name": "bloodtestReportIpfsHash",
          "type": "string"
        }
      ]
    },
    {
      "name": "updateGcReport",
      "accounts": [
        {
          "name": "generalConsultancy",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "doctorProfile",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "doctor",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "bloodPressure",
          "type": "u32"
        },
        {
          "name": "bloodSugar",
          "type": "u32"
        },
        {
          "name": "temperature",
          "type": "u32"
        },
        {
          "name": "doctorComments",
          "type": "string"
        },
        {
          "name": "gcReportIpfsHash",
          "type": "string"
        }
      ]
    },
    {
      "name": "verifyBloodtestReport",
      "accounts": [
        {
          "name": "bloodtestReport",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "patientProfile",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patientDepositTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "technicianReceiveTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "daoTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patient",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "verifyGcReport",
      "accounts": [
        {
          "name": "generalConsultancy",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "patientProfile",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "patientDepositTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "doctorReceiveTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "daoTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patient",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "bloodtestReport",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "patient",
            "type": "publicKey"
          },
          {
            "name": "technician",
            "type": "publicKey"
          },
          {
            "name": "patientDepositTokenAccount",
            "type": "publicKey"
          },
          {
            "name": "technicianBloodtestFee",
            "type": "u64"
          },
          {
            "name": "patientBookingTime",
            "type": "i64"
          },
          {
            "name": "technicianBloodtestExpiryTime",
            "type": "i64"
          },
          {
            "name": "uhwDaoShare",
            "type": "u64"
          },
          {
            "name": "redBloodCells",
            "type": "u128"
          },
          {
            "name": "whiteBloodCells",
            "type": "u128"
          },
          {
            "name": "bloodPlatelets",
            "type": "u128"
          },
          {
            "name": "technicianComments",
            "type": "string"
          },
          {
            "name": "bloodtestReportIpfsHash",
            "type": "string"
          },
          {
            "name": "patientVerified",
            "type": "bool"
          },
          {
            "name": "bloodtestReportBump",
            "type": "u8"
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                4
              ]
            }
          }
        ]
      }
    },
    {
      "name": "doctorProfile",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "doctor",
            "type": "publicKey"
          },
          {
            "name": "doctorName",
            "type": "string"
          },
          {
            "name": "doctorSex",
            "type": "string"
          },
          {
            "name": "doctorDob",
            "type": "string"
          },
          {
            "name": "doctorExperienceMonths",
            "type": "u32"
          },
          {
            "name": "doctorGcFee",
            "type": "u64"
          },
          {
            "name": "doctorGcExpiryTime",
            "type": "i64"
          },
          {
            "name": "signUpDate",
            "type": "i64"
          },
          {
            "name": "doctorLicence",
            "type": "bool"
          },
          {
            "name": "doctorProfileBump",
            "type": "u8"
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                6
              ]
            }
          }
        ]
      }
    },
    {
      "name": "generalConsultancy",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "patient",
            "type": "publicKey"
          },
          {
            "name": "doctor",
            "type": "publicKey"
          },
          {
            "name": "patientDepositTokenAccount",
            "type": "publicKey"
          },
          {
            "name": "doctorGcFee",
            "type": "u64"
          },
          {
            "name": "bookingTime",
            "type": "i64"
          },
          {
            "name": "generalConsultancyExpiryTime",
            "type": "i64"
          },
          {
            "name": "uhwDaoShare",
            "type": "u64"
          },
          {
            "name": "patientComments",
            "type": "string"
          },
          {
            "name": "bloodPressure",
            "type": "u32"
          },
          {
            "name": "bloodSugar",
            "type": "u32"
          },
          {
            "name": "temperature",
            "type": "u32"
          },
          {
            "name": "doctorComments",
            "type": "string"
          },
          {
            "name": "gcReportIpfsHash",
            "type": "string"
          },
          {
            "name": "patientVerified",
            "type": "bool"
          },
          {
            "name": "generalConsultancyBump",
            "type": "u8"
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                6
              ]
            }
          }
        ]
      }
    },
    {
      "name": "patientProfile",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "patient",
            "type": "publicKey"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "sex",
            "type": "string"
          },
          {
            "name": "dob",
            "type": "string"
          },
          {
            "name": "signupDate",
            "type": "i64"
          },
          {
            "name": "height",
            "type": "u32"
          },
          {
            "name": "weight",
            "type": "u32"
          },
          {
            "name": "patientProfileBump",
            "type": "u8"
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                7
              ]
            }
          }
        ]
      }
    },
    {
      "name": "technicianProfile",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "technician",
            "type": "publicKey"
          },
          {
            "name": "technicianName",
            "type": "string"
          },
          {
            "name": "technicianSex",
            "type": "string"
          },
          {
            "name": "technicianDob",
            "type": "string"
          },
          {
            "name": "technicianExperienceMonths",
            "type": "u32"
          },
          {
            "name": "technicianBloodtestFee",
            "type": "u64"
          },
          {
            "name": "technicianBloodtestExpiryTime",
            "type": "i64"
          },
          {
            "name": "signUpDate",
            "type": "i64"
          },
          {
            "name": "technicianLicence",
            "type": "bool"
          },
          {
            "name": "technicianProfileBump",
            "type": "u8"
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                6
              ]
            }
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "NotEnoughBalance",
      "msg": "Src Balance < LP Deposit Amount."
    },
    {
      "code": 6001,
      "name": "NameTooLong",
      "msg": "Name is more than 128 bytes"
    },
    {
      "code": 6002,
      "name": "DOBTooLong",
      "msg": "DOB is more than 16 bytes"
    },
    {
      "code": 6003,
      "name": "SexTooLong",
      "msg": "Sex is more than 16 bytes"
    },
    {
      "code": 6004,
      "name": "PatientCommentsLong",
      "msg": "Patient Comments is more than 512 bytes"
    },
    {
      "code": 6005,
      "name": "MathOverflow",
      "msg": "Math Overflow"
    }
  ]
};

export const IDL: Ehr = {
  "version": "0.1.0",
  "name": "ehr",
  "instructions": [
    {
      "name": "initBloodtestBooking",
      "accounts": [
        {
          "name": "bloodtestReport",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patientProfile",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "technicianProfile",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patientDepositTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patient",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initDoctorProfile",
      "accounts": [
        {
          "name": "doctorProfile",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "doctor",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "doctorName",
          "type": "string"
        },
        {
          "name": "doctorSex",
          "type": "string"
        },
        {
          "name": "doctorDob",
          "type": "string"
        },
        {
          "name": "doctorExperienceMonths",
          "type": "u32"
        },
        {
          "name": "doctorGcFee",
          "type": "u64"
        },
        {
          "name": "doctorLicence",
          "type": "bool"
        },
        {
          "name": "doctorGcExpiryTime",
          "type": "i64"
        }
      ]
    },
    {
      "name": "initGcBooking",
      "accounts": [
        {
          "name": "generalConsultancy",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patientProfile",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "doctorProfile",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patientDepositTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patient",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "patientComments",
          "type": "string"
        }
      ]
    },
    {
      "name": "initPatientProfile",
      "accounts": [
        {
          "name": "patientProfile",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patient",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "sex",
          "type": "string"
        },
        {
          "name": "dob",
          "type": "string"
        }
      ]
    },
    {
      "name": "initTechnicianProfile",
      "accounts": [
        {
          "name": "technicianProfile",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "technician",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "technicianName",
          "type": "string"
        },
        {
          "name": "technicianSex",
          "type": "string"
        },
        {
          "name": "technicianDob",
          "type": "string"
        },
        {
          "name": "technicianExperienceMonths",
          "type": "u32"
        },
        {
          "name": "technicianBloodtestFee",
          "type": "u64"
        },
        {
          "name": "technicianLicence",
          "type": "bool"
        },
        {
          "name": "technicianBloodtestExpiryTime",
          "type": "i64"
        }
      ]
    },
    {
      "name": "updateBloodtestReport",
      "accounts": [
        {
          "name": "bloodtestReport",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "technicianProfile",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "technician",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "whiteBloodCells",
          "type": "u128"
        },
        {
          "name": "redBloodCells",
          "type": "u128"
        },
        {
          "name": "bloodPlatelets",
          "type": "u128"
        },
        {
          "name": "technicianComments",
          "type": "string"
        },
        {
          "name": "bloodtestReportIpfsHash",
          "type": "string"
        }
      ]
    },
    {
      "name": "updateGcReport",
      "accounts": [
        {
          "name": "generalConsultancy",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "doctorProfile",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "doctor",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "bloodPressure",
          "type": "u32"
        },
        {
          "name": "bloodSugar",
          "type": "u32"
        },
        {
          "name": "temperature",
          "type": "u32"
        },
        {
          "name": "doctorComments",
          "type": "string"
        },
        {
          "name": "gcReportIpfsHash",
          "type": "string"
        }
      ]
    },
    {
      "name": "verifyBloodtestReport",
      "accounts": [
        {
          "name": "bloodtestReport",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "patientProfile",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patientDepositTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "technicianReceiveTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "daoTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patient",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "verifyGcReport",
      "accounts": [
        {
          "name": "generalConsultancy",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "patientProfile",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "patientDepositTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "doctorReceiveTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "daoTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "patient",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "bloodtestReport",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "patient",
            "type": "publicKey"
          },
          {
            "name": "technician",
            "type": "publicKey"
          },
          {
            "name": "patientDepositTokenAccount",
            "type": "publicKey"
          },
          {
            "name": "technicianBloodtestFee",
            "type": "u64"
          },
          {
            "name": "patientBookingTime",
            "type": "i64"
          },
          {
            "name": "technicianBloodtestExpiryTime",
            "type": "i64"
          },
          {
            "name": "uhwDaoShare",
            "type": "u64"
          },
          {
            "name": "redBloodCells",
            "type": "u128"
          },
          {
            "name": "whiteBloodCells",
            "type": "u128"
          },
          {
            "name": "bloodPlatelets",
            "type": "u128"
          },
          {
            "name": "technicianComments",
            "type": "string"
          },
          {
            "name": "bloodtestReportIpfsHash",
            "type": "string"
          },
          {
            "name": "patientVerified",
            "type": "bool"
          },
          {
            "name": "bloodtestReportBump",
            "type": "u8"
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                4
              ]
            }
          }
        ]
      }
    },
    {
      "name": "doctorProfile",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "doctor",
            "type": "publicKey"
          },
          {
            "name": "doctorName",
            "type": "string"
          },
          {
            "name": "doctorSex",
            "type": "string"
          },
          {
            "name": "doctorDob",
            "type": "string"
          },
          {
            "name": "doctorExperienceMonths",
            "type": "u32"
          },
          {
            "name": "doctorGcFee",
            "type": "u64"
          },
          {
            "name": "doctorGcExpiryTime",
            "type": "i64"
          },
          {
            "name": "signUpDate",
            "type": "i64"
          },
          {
            "name": "doctorLicence",
            "type": "bool"
          },
          {
            "name": "doctorProfileBump",
            "type": "u8"
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                6
              ]
            }
          }
        ]
      }
    },
    {
      "name": "generalConsultancy",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "patient",
            "type": "publicKey"
          },
          {
            "name": "doctor",
            "type": "publicKey"
          },
          {
            "name": "patientDepositTokenAccount",
            "type": "publicKey"
          },
          {
            "name": "doctorGcFee",
            "type": "u64"
          },
          {
            "name": "bookingTime",
            "type": "i64"
          },
          {
            "name": "generalConsultancyExpiryTime",
            "type": "i64"
          },
          {
            "name": "uhwDaoShare",
            "type": "u64"
          },
          {
            "name": "patientComments",
            "type": "string"
          },
          {
            "name": "bloodPressure",
            "type": "u32"
          },
          {
            "name": "bloodSugar",
            "type": "u32"
          },
          {
            "name": "temperature",
            "type": "u32"
          },
          {
            "name": "doctorComments",
            "type": "string"
          },
          {
            "name": "gcReportIpfsHash",
            "type": "string"
          },
          {
            "name": "patientVerified",
            "type": "bool"
          },
          {
            "name": "generalConsultancyBump",
            "type": "u8"
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                6
              ]
            }
          }
        ]
      }
    },
    {
      "name": "patientProfile",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "patient",
            "type": "publicKey"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "sex",
            "type": "string"
          },
          {
            "name": "dob",
            "type": "string"
          },
          {
            "name": "signupDate",
            "type": "i64"
          },
          {
            "name": "height",
            "type": "u32"
          },
          {
            "name": "weight",
            "type": "u32"
          },
          {
            "name": "patientProfileBump",
            "type": "u8"
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                7
              ]
            }
          }
        ]
      }
    },
    {
      "name": "technicianProfile",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "technician",
            "type": "publicKey"
          },
          {
            "name": "technicianName",
            "type": "string"
          },
          {
            "name": "technicianSex",
            "type": "string"
          },
          {
            "name": "technicianDob",
            "type": "string"
          },
          {
            "name": "technicianExperienceMonths",
            "type": "u32"
          },
          {
            "name": "technicianBloodtestFee",
            "type": "u64"
          },
          {
            "name": "technicianBloodtestExpiryTime",
            "type": "i64"
          },
          {
            "name": "signUpDate",
            "type": "i64"
          },
          {
            "name": "technicianLicence",
            "type": "bool"
          },
          {
            "name": "technicianProfileBump",
            "type": "u8"
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                6
              ]
            }
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "NotEnoughBalance",
      "msg": "Src Balance < LP Deposit Amount."
    },
    {
      "code": 6001,
      "name": "NameTooLong",
      "msg": "Name is more than 128 bytes"
    },
    {
      "code": 6002,
      "name": "DOBTooLong",
      "msg": "DOB is more than 16 bytes"
    },
    {
      "code": 6003,
      "name": "SexTooLong",
      "msg": "Sex is more than 16 bytes"
    },
    {
      "code": 6004,
      "name": "PatientCommentsLong",
      "msg": "Patient Comments is more than 512 bytes"
    },
    {
      "code": 6005,
      "name": "MathOverflow",
      "msg": "Math Overflow"
    }
  ]
};
