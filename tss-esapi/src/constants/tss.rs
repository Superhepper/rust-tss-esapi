// Copyright 2019 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
use crate::tss2_esys::*;

pub const TPM2_ALG_ERROR: TPM2_ALG_ID = 0x0000;
pub const TPM2_ALG_RSA: TPM2_ALG_ID = 0x0001;
pub const TPM2_ALG_TDES: TPM2_ALG_ID = 0x0003;
pub const TPM2_ALG_SHA: TPM2_ALG_ID = 0x0004;
pub const TPM2_ALG_SHA1: TPM2_ALG_ID = 0x0004;
pub const TPM2_ALG_HMAC: TPM2_ALG_ID = 0x0005;
pub const TPM2_ALG_AES: TPM2_ALG_ID = 0x0006;
pub const TPM2_ALG_MGF1: TPM2_ALG_ID = 0x0007;
pub const TPM2_ALG_KEYEDHASH: TPM2_ALG_ID = 0x0008;
pub const TPM2_ALG_XOR: TPM2_ALG_ID = 0x000A;
pub const TPM2_ALG_SHA256: TPM2_ALG_ID = 0x000B;
pub const TPM2_ALG_SHA384: TPM2_ALG_ID = 0x000C;
pub const TPM2_ALG_SHA512: TPM2_ALG_ID = 0x000D;
pub const TPM2_ALG_NULL: TPM2_ALG_ID = 0x0010;
pub const TPM2_ALG_SM3_256: TPM2_ALG_ID = 0x0012;
pub const TPM2_ALG_SM4: TPM2_ALG_ID = 0x0013;
pub const TPM2_ALG_RSASSA: TPM2_ALG_ID = 0x0014;
pub const TPM2_ALG_RSAES: TPM2_ALG_ID = 0x0015;
pub const TPM2_ALG_RSAPSS: TPM2_ALG_ID = 0x0016;
pub const TPM2_ALG_OAEP: TPM2_ALG_ID = 0x0017;
pub const TPM2_ALG_ECDSA: TPM2_ALG_ID = 0x0018;
pub const TPM2_ALG_ECDH: TPM2_ALG_ID = 0x0019;
pub const TPM2_ALG_ECDAA: TPM2_ALG_ID = 0x001A;
pub const TPM2_ALG_SM2: TPM2_ALG_ID = 0x001B;
pub const TPM2_ALG_ECSCHNORR: TPM2_ALG_ID = 0x001C;
pub const TPM2_ALG_ECMQV: TPM2_ALG_ID = 0x001D;
pub const TPM2_ALG_KDF1_SP800_56A: TPM2_ALG_ID = 0x0020;
pub const TPM2_ALG_KDF2: TPM2_ALG_ID = 0x0021;
pub const TPM2_ALG_KDF1_SP800_108: TPM2_ALG_ID = 0x0022;
pub const TPM2_ALG_ECC: TPM2_ALG_ID = 0x0023;
pub const TPM2_ALG_SYMCIPHER: TPM2_ALG_ID = 0x0025;
pub const TPM2_ALG_CAMELLIA: TPM2_ALG_ID = 0x0026;
pub const TPM2_ALG_CMAC: TPM2_ALG_ID = 0x003F;
pub const TPM2_ALG_CTR: TPM2_ALG_ID = 0x0040;
pub const TPM2_ALG_SHA3_256: TPM2_ALG_ID = 0x0027;
pub const TPM2_ALG_SHA3_384: TPM2_ALG_ID = 0x0028;
pub const TPM2_ALG_SHA3_512: TPM2_ALG_ID = 0x0029;
pub const TPM2_ALG_OFB: TPM2_ALG_ID = 0x0041;
pub const TPM2_ALG_CBC: TPM2_ALG_ID = 0x0042;
pub const TPM2_ALG_CFB: TPM2_ALG_ID = 0x0043;
pub const TPM2_ALG_ECB: TPM2_ALG_ID = 0x0044;
pub const TPM2_ALG_FIRST: TPM2_ALG_ID = 0x0001;
pub const TPM2_ALG_LAST: TPM2_ALG_ID = 0x0044;

pub const TPM2_ECC_NONE: TPM2_ECC_CURVE = 0x0000;
pub const TPM2_ECC_NIST_P192: TPM2_ECC_CURVE = 0x0001;
pub const TPM2_ECC_NIST_P224: TPM2_ECC_CURVE = 0x0002;
pub const TPM2_ECC_NIST_P256: TPM2_ECC_CURVE = 0x0003;
pub const TPM2_ECC_NIST_P384: TPM2_ECC_CURVE = 0x0004;
pub const TPM2_ECC_NIST_P521: TPM2_ECC_CURVE = 0x0005;
pub const TPM2_ECC_BN_P256: TPM2_ECC_CURVE = 0x0010;
pub const TPM2_ECC_BN_P638: TPM2_ECC_CURVE = 0x0011;
pub const TPM2_ECC_SM2_P256: TPM2_ECC_CURVE = 0x0020;

pub const TPM2_CC_NV_UndefineSpaceSpecial: TPM2_CC = 0x0000011f;
pub const TPM2_CC_FIRST: TPM2_CC = TPM2_CC_NV_UndefineSpaceSpecial;
pub const TPM2_CC_EvictControl: TPM2_CC = 0x00000120;
pub const TPM2_CC_HierarchyControl: TPM2_CC = 0x00000121;
pub const TPM2_CC_NV_UndefineSpace: TPM2_CC = 0x00000122;
pub const TPM2_CC_ChangeEPS: TPM2_CC = 0x00000124;
pub const TPM2_CC_ChangePPS: TPM2_CC = 0x00000125;
pub const TPM2_CC_Clear: TPM2_CC = 0x00000126;
pub const TPM2_CC_ClearControl: TPM2_CC = 0x00000127;
pub const TPM2_CC_ClockSet: TPM2_CC = 0x00000128;
pub const TPM2_CC_HierarchyChangeAuth: TPM2_CC = 0x00000129;
pub const TPM2_CC_NV_DefineSpace: TPM2_CC = 0x0000012a;
pub const TPM2_CC_PCR_Allocate: TPM2_CC = 0x0000012b;
pub const TPM2_CC_PCR_SetAuthPolicy: TPM2_CC = 0x00000012c;
pub const TPM2_CC_PP_Commands: TPM2_CC = 0x0000012d;
pub const TPM2_CC_SetPrimaryPolicy: TPM2_CC = 0x0000012e;
pub const TPM2_CC_FieldUpgradeStart: TPM2_CC = 0x0000012f;
pub const TPM2_CC_ClockRateAdjust: TPM2_CC = 0x00000130;
pub const TPM2_CC_CreatePrimary: TPM2_CC = 0x00000131;
pub const TPM2_CC_NV_GlobalWriteLock: TPM2_CC = 0x00000132;
pub const TPM2_CC_GetCommandAuditDigest: TPM2_CC = 0x00000133;
pub const TPM2_CC_NV_Increment: TPM2_CC = 0x00000134;
pub const TPM2_CC_NV_SetBits: TPM2_CC = 0x00000135;
pub const TPM2_CC_NV_Extend: TPM2_CC = 0x00000136;
pub const TPM2_CC_NV_Write: TPM2_CC = 0x00000137;
pub const TPM2_CC_NV_WriteLock: TPM2_CC = 0x00000138;
pub const TPM2_CC_DictionaryAttackLockReset: TPM2_CC = 0x00000139;
pub const TPM2_CC_DictionaryAttackParameters: TPM2_CC = 0x0000013a;
pub const TPM2_CC_NV_ChangeAuth: TPM2_CC = 0x0000013b;
pub const TPM2_CC_PCR_Event: TPM2_CC = 0x0000013c;
pub const TPM2_CC_PCR_Reset: TPM2_CC = 0x0000013d;
pub const TPM2_CC_SequenceComplete: TPM2_CC = 0x0000013e;
pub const TPM2_CC_SetAlgorithmSet: TPM2_CC = 0x0000013f;
pub const TPM2_CC_SetCommandCodeAuditStatus: TPM2_CC = 0x00000140;
pub const TPM2_CC_FieldUpgradeData: TPM2_CC = 0x00000141;
pub const TPM2_CC_IncrementalSelfTest: TPM2_CC = 0x00000142;
pub const TPM2_CC_SelfTest: TPM2_CC = 0x00000143;
pub const TPM2_CC_Startup: TPM2_CC = 0x00000144;
pub const TPM2_CC_Shutdown: TPM2_CC = 0x00000145;
pub const TPM2_CC_StirRandom: TPM2_CC = 0x00000146;
pub const TPM2_CC_ActivateCredential: TPM2_CC = 0x00000147;
pub const TPM2_CC_Certify: TPM2_CC = 0x00000148;
pub const TPM2_CC_PolicyNV: TPM2_CC = 0x00000149;
pub const TPM2_CC_CertifyCreation: TPM2_CC = 0x0000014a;
pub const TPM2_CC_Duplicate: TPM2_CC = 0x0000014b;
pub const TPM2_CC_GetTime: TPM2_CC = 0x0000014c;
pub const TPM2_CC_GetSessionAuditDigest: TPM2_CC = 0x0000014d;
pub const TPM2_CC_NV_Read: TPM2_CC = 0x0000014e;
pub const TPM2_CC_NV_ReadLock: TPM2_CC = 0x0000014f;
pub const TPM2_CC_ObjectChangeAuth: TPM2_CC = 0x00000150;
pub const TPM2_CC_PolicySecret: TPM2_CC = 0x00000151;
pub const TPM2_CC_Rewrap: TPM2_CC = 0x00000152;
pub const TPM2_CC_Create: TPM2_CC = 0x00000153;
pub const TPM2_CC_ECDH_ZGen: TPM2_CC = 0x00000154;
pub const TPM2_CC_HMAC: TPM2_CC = 0x00000155;
pub const TPM2_CC_Import: TPM2_CC = 0x00000156;
pub const TPM2_CC_Load: TPM2_CC = 0x00000157;
pub const TPM2_CC_Quote: TPM2_CC = 0x00000158;
pub const TPM2_CC_RSA_Decrypt: TPM2_CC = 0x00000159;
pub const TPM2_CC_HMAC_Start: TPM2_CC = 0x0000015b;
pub const TPM2_CC_SequenceUpdate: TPM2_CC = 0x0000015c;
pub const TPM2_CC_Sign: TPM2_CC = 0x0000015d;
pub const TPM2_CC_Unseal: TPM2_CC = 0x0000015e;
pub const TPM2_CC_PolicySigned: TPM2_CC = 0x00000160;
pub const TPM2_CC_ContextLoad: TPM2_CC = 0x00000161;
pub const TPM2_CC_ContextSave: TPM2_CC = 0x00000162;
pub const TPM2_CC_ECDH_KeyGen: TPM2_CC = 0x00000163;
pub const TPM2_CC_EncryptDecrypt: TPM2_CC = 0x00000164;
pub const TPM2_CC_FlushContext: TPM2_CC = 0x00000165;
pub const TPM2_CC_LoadExternal: TPM2_CC = 0x00000167;
pub const TPM2_CC_MakeCredential: TPM2_CC = 0x00000168;
pub const TPM2_CC_NV_ReadPublic: TPM2_CC = 0x00000169;
pub const TPM2_CC_PolicyAuthorize: TPM2_CC = 0x0000016a;
pub const TPM2_CC_PolicyAuthValue: TPM2_CC = 0x0000016b;
pub const TPM2_CC_PolicyCommandCode: TPM2_CC = 0x0000016c;
pub const TPM2_CC_PolicyCounterTimer: TPM2_CC = 0x0000016d;
pub const TPM2_CC_PolicyCpHash: TPM2_CC = 0x0000016e;
pub const TPM2_CC_PolicyLocality: TPM2_CC = 0x0000016f;
pub const TPM2_CC_PolicyNameHash: TPM2_CC = 0x00000170;
pub const TPM2_CC_PolicyOR: TPM2_CC = 0x00000171;
pub const TPM2_CC_PolicyTicket: TPM2_CC = 0x00000172;
pub const TPM2_CC_ReadPublic: TPM2_CC = 0x00000173;
pub const TPM2_CC_RSA_Encrypt: TPM2_CC = 0x00000174;
pub const TPM2_CC_StartAuthSession: TPM2_CC = 0x00000176;
pub const TPM2_CC_VerifySignature: TPM2_CC = 0x00000177;
pub const TPM2_CC_ECC_Parameters: TPM2_CC = 0x00000178;
pub const TPM2_CC_FirmwareRead: TPM2_CC = 0x00000179;
pub const TPM2_CC_GetCapability: TPM2_CC = 0x0000017a;
pub const TPM2_CC_GetRandom: TPM2_CC = 0x0000017b;
pub const TPM2_CC_GetTestResult: TPM2_CC = 0x0000017c;
pub const TPM2_CC_Hash: TPM2_CC = 0x0000017d;
pub const TPM2_CC_PCR_Read: TPM2_CC = 0x0000017e;
pub const TPM2_CC_PolicyPCR: TPM2_CC = 0x0000017f;
pub const TPM2_CC_PolicyRestart: TPM2_CC = 0x00000180;
pub const TPM2_CC_ReadClock: TPM2_CC = 0x00000181;
pub const TPM2_CC_PCR_Extend: TPM2_CC = 0x00000182;
pub const TPM2_CC_PCR_SetAuthValue: TPM2_CC = 0x00000183;
pub const TPM2_CC_NV_Certify: TPM2_CC = 0x00000184;
pub const TPM2_CC_EventSequenceComplete: TPM2_CC = 0x00000185;
pub const TPM2_CC_HashSequenceStart: TPM2_CC = 0x00000186;
pub const TPM2_CC_PolicyPhysicalPresence: TPM2_CC = 0x00000187;
pub const TPM2_CC_PolicyDuplicationSelect: TPM2_CC = 0x00000188;
pub const TPM2_CC_PolicyGetDigest: TPM2_CC = 0x00000189;
pub const TPM2_CC_TestParms: TPM2_CC = 0x0000018a;
pub const TPM2_CC_Commit: TPM2_CC = 0x0000018b;
pub const TPM2_CC_PolicyPassword: TPM2_CC = 0x0000018c;
pub const TPM2_CC_ZGen_2Phase: TPM2_CC = 0x0000018d;
pub const TPM2_CC_EC_Ephemeral: TPM2_CC = 0x0000018e;
pub const TPM2_CC_PolicyNvWritten: TPM2_CC = 0x0000018f;
pub const TPM2_CC_PolicyTemplate: TPM2_CC = 0x00000190;
pub const TPM2_CC_CreateLoaded: TPM2_CC = 0x00000191;
pub const TPM2_CC_PolicyAuthorizeNV: TPM2_CC = 0x00000192;
pub const TPM2_CC_EncryptDecrypt2: TPM2_CC = 0x00000193;
pub const TPM2_CC_AC_GetCapability: TPM2_CC = 0x00000194;
pub const TPM2_CC_AC_Send: TPM2_CC = 0x00000195;
pub const TPM2_CC_Policy_AC_SendSelect: TPM2_CC = 0x00000196;
pub const TPM2_CC_LAST: TPM2_CC = 0x00000196;
pub const TPM2_CC_Vendor_TCG_Test: TPM2_CC = 0x20000000;

pub const TPM2_SPEC_FAMILY: TPM2_SPEC = 0x322E3000; /* ASCII 2.0 with null terminator */
pub const TPM2_SPEC_LEVEL: TPM2_SPEC = 00; /* the level number for the specification */
pub const TPM2_SPEC_VERSION: TPM2_SPEC = 126; /* the version number of the spec 001.26 * 100 */
pub const TPM2_SPEC_YEAR: TPM2_SPEC = 2015; /* the year of the version */
pub const TPM2_SPEC_DAY_OF_YEAR: TPM2_SPEC = 233; /* the day of the year August 21 2015 */

pub const TPM2_GENERATED_VALUE: TPM2_GENERATED = 0xff544347; /* 0xFF TCG FF 54 43 4716 */

pub const TPM2_RC_SUCCESS: TPM2_RC = 0x000;
pub const TPM2_RC_BAD_TAG: TPM2_RC = 0x01E; /* defined for compatibility with TPM 1.2 */
pub const TPM2_RC_VER1: TPM2_RC = 0x100; /* set for all format 0 response codes */
pub const TPM2_RC_INITIALIZE: TPM2_RC = TPM2_RC_VER1 + 0x000; /* TPM not initialized by TPM2_Startup or already initialized */
pub const TPM2_RC_FAILURE: TPM2_RC = TPM2_RC_VER1 + 0x001; /* commands not being accepted because of a TPM failure. NOTE This may be returned by TPM2_GetTestResult as the testResult parameter. */
pub const TPM2_RC_SEQUENCE: TPM2_RC = TPM2_RC_VER1 + 0x003; /* improper use of a sequence handle */
pub const TPM2_RC_PRIVATE: TPM2_RC = TPM2_RC_VER1 + 0x00B; /* not currently used */
pub const TPM2_RC_HMAC: TPM2_RC = TPM2_RC_VER1 + 0x019; /* not currently used */
pub const TPM2_RC_DISABLED: TPM2_RC = TPM2_RC_VER1 + 0x020; /* the command is disabled */
pub const TPM2_RC_EXCLUSIVE: TPM2_RC = TPM2_RC_VER1 + 0x021; /* command failed because audit sequence required exclusivity */
pub const TPM2_RC_AUTH_TYPE: TPM2_RC = TPM2_RC_VER1 + 0x024; /* authorization handle is not correct for command */
pub const TPM2_RC_AUTH_MISSING: TPM2_RC = TPM2_RC_VER1 + 0x025; /* command requires an authorization session for handle and it is not present. */
pub const TPM2_RC_POLICY: TPM2_RC = TPM2_RC_VER1 + 0x026; /* policy failure in math operation or an invalid authPolicy value */
pub const TPM2_RC_PCR: TPM2_RC = TPM2_RC_VER1 + 0x027; /* PCR check fail */
pub const TPM2_RC_PCR_CHANGED: TPM2_RC = TPM2_RC_VER1 + 0x028; /* PCR have changed since checked. */
pub const TPM2_RC_UPGRADE: TPM2_RC = TPM2_RC_VER1 + 0x02D; /* For all commands, other than TPM2_FieldUpgradeData, this code indicates that the TPM is in field upgrade mode. For TPM2_FieldUpgradeData, this code indicates that the TPM is not in field upgrade mode */
pub const TPM2_RC_TOO_MANY_CONTEXTS: TPM2_RC = TPM2_RC_VER1 + 0x02E; /* context ID counter is at maximum. */
pub const TPM2_RC_AUTH_UNAVAILABLE: TPM2_RC = TPM2_RC_VER1 + 0x02F; /* authValue or authPolicy is not available for selected entity. */
pub const TPM2_RC_REBOOT: TPM2_RC = TPM2_RC_VER1 + 0x030; /* a _TPM_Init and StartupCLEAR is required before the TPM can resume operation. */
pub const TPM2_RC_UNBALANCED: TPM2_RC = TPM2_RC_VER1 + 0x031; /* the protection algorithms hash and symmetric are not reasonably balanced. The digest size of the hash must be larger than the key size of the symmetric algorithm. */
pub const TPM2_RC_COMMAND_SIZE: TPM2_RC = TPM2_RC_VER1 + 0x042; /* command commandSize value is inconsistent with contents of the command buffer. Either the size is not the same as the octets loaded by the hardware interface layer or the value is not large enough to hold a command header */
pub const TPM2_RC_COMMAND_CODE: TPM2_RC = TPM2_RC_VER1 + 0x043; /* command code not supported */
pub const TPM2_RC_AUTHSIZE: TPM2_RC = TPM2_RC_VER1 + 0x044; /* the value of authorizationSize is out of range or the number of octets in the Authorization Area is greater than required */
pub const TPM2_RC_AUTH_CONTEXT: TPM2_RC = TPM2_RC_VER1 + 0x045; /* use of an authorization session with a context command or another command that cannot have an authorization session. */
pub const TPM2_RC_NV_RANGE: TPM2_RC = TPM2_RC_VER1 + 0x046; /* NV offset+size is out of range. */
pub const TPM2_RC_NV_SIZE: TPM2_RC = TPM2_RC_VER1 + 0x047; /* Requested allocation size is larger than allowed. */
pub const TPM2_RC_NV_LOCKED: TPM2_RC = TPM2_RC_VER1 + 0x048; /* NV access locked. */
pub const TPM2_RC_NV_AUTHORIZATION: TPM2_RC = TPM2_RC_VER1 + 0x049; /* NV access authorization fails in command actions this failure does not affect lockout.action */
pub const TPM2_RC_NV_UNINITIALIZED: TPM2_RC = TPM2_RC_VER1 + 0x04A; /* an NV Index is used before being initialized or the state saved by TPM2_ShutdownSTATE could not be restored */
pub const TPM2_RC_NV_SPACE: TPM2_RC = TPM2_RC_VER1 + 0x04B; /* insufficient space for NV allocation */
pub const TPM2_RC_NV_DEFINED: TPM2_RC = TPM2_RC_VER1 + 0x04C; /* NV Index or persistent object already defined */
pub const TPM2_RC_BAD_CONTEXT: TPM2_RC = TPM2_RC_VER1 + 0x050; /* context in TPM2_ContextLoad is not valid */
pub const TPM2_RC_CPHASH: TPM2_RC = TPM2_RC_VER1 + 0x051; /* cpHash value already set or not correct for use */
pub const TPM2_RC_PARENT: TPM2_RC = TPM2_RC_VER1 + 0x052; /* handle for parent is not a valid parent */
pub const TPM2_RC_NEEDS_TEST: TPM2_RC = TPM2_RC_VER1 + 0x053; /* some function needs testing. */
pub const TPM2_RC_NO_RESULT: TPM2_RC = TPM2_RC_VER1 + 0x054; /* returned when an internal function cannot process a request due to an unspecified problem. This code is usually related to invalid parameters that are not properly filtered by the input unmarshaling code. */
pub const TPM2_RC_SENSITIVE: TPM2_RC = TPM2_RC_VER1 + 0x055; /* the sensitive area did not unmarshal correctly after decryption. This code is used in lieu of the other unmarshaling errors so that an attacker cannot determine where the unmarshaling error occurred */
pub const TPM2_RC_MAX_FM0: TPM2_RC = TPM2_RC_VER1 + 0x07F; /* largest version 1 code that is not a warning */
pub const TPM2_RC_FMT1: TPM2_RC = 0x080; /* This bit is SET in all format 1 response codes. The codes in this group may have a value added to them to indicate the handle session or parameter to which they apply. */
pub const TPM2_RC_ASYMMETRIC: TPM2_RC = TPM2_RC_FMT1 + 0x001; /* asymmetric algorithm not supported or not correct */
pub const TPM2_RC_ATTRIBUTES: TPM2_RC = TPM2_RC_FMT1 + 0x002; /* inconsistent attributes */
pub const TPM2_RC_HASH: TPM2_RC = TPM2_RC_FMT1 + 0x003; /* hash algorithm not supported or not appropriate */
pub const TPM2_RC_VALUE: TPM2_RC = TPM2_RC_FMT1 + 0x004; /* value is out of range or is not correct for the context */
pub const TPM2_RC_HIERARCHY: TPM2_RC = TPM2_RC_FMT1 + 0x005; /* hierarchy is not enabled or is not correct for the use */
pub const TPM2_RC_KEY_SIZE: TPM2_RC = TPM2_RC_FMT1 + 0x007; /* key size is not supported */
pub const TPM2_RC_MGF: TPM2_RC = TPM2_RC_FMT1 + 0x008; /* mask generation function not supported */
pub const TPM2_RC_MODE: TPM2_RC = TPM2_RC_FMT1 + 0x009; /* mode of operation not supported */
pub const TPM2_RC_TYPE: TPM2_RC = TPM2_RC_FMT1 + 0x00A; /* the type of the value is not appropriate for the use */
pub const TPM2_RC_HANDLE: TPM2_RC = TPM2_RC_FMT1 + 0x00B; /* the handle is not correct for the use */
pub const TPM2_RC_KDF: TPM2_RC = TPM2_RC_FMT1 + 0x00C; /* unsupported key derivation function or function not appropriate for use */
pub const TPM2_RC_RANGE: TPM2_RC = TPM2_RC_FMT1 + 0x00D; /* value was out of allowed range. */
pub const TPM2_RC_AUTH_FAIL: TPM2_RC = TPM2_RC_FMT1 + 0x00E; /* the authorization HMAC check failed and DA counter incremented */
pub const TPM2_RC_NONCE: TPM2_RC = TPM2_RC_FMT1 + 0x00F; /* invalid nonce size or nonce value mismatch */
pub const TPM2_RC_PP: TPM2_RC = TPM2_RC_FMT1 + 0x010; /* authorization requires assertion of PP */
pub const TPM2_RC_SCHEME: TPM2_RC = TPM2_RC_FMT1 + 0x012; /* unsupported or incompatible scheme */
pub const TPM2_RC_SIZE: TPM2_RC = TPM2_RC_FMT1 + 0x015; /* structure is the wrong size */
pub const TPM2_RC_SYMMETRIC: TPM2_RC = TPM2_RC_FMT1 + 0x016; /* unsupported symmetric algorithm or key size or not appropriate for instance */
pub const TPM2_RC_TAG: TPM2_RC = TPM2_RC_FMT1 + 0x017; /* incorrect structure tag */
pub const TPM2_RC_SELECTOR: TPM2_RC = TPM2_RC_FMT1 + 0x018; /* union selector is incorrect */
pub const TPM2_RC_INSUFFICIENT: TPM2_RC = TPM2_RC_FMT1 + 0x01A; /* the TPM was unable to unmarshal a value because there were not enough octets in the input buffer */
pub const TPM2_RC_SIGNATURE: TPM2_RC = TPM2_RC_FMT1 + 0x01B; /* the signature is not valid */
pub const TPM2_RC_KEY: TPM2_RC = TPM2_RC_FMT1 + 0x01C; /* key fields are not compatible with the selected use */
pub const TPM2_RC_POLICY_FAIL: TPM2_RC = TPM2_RC_FMT1 + 0x01D; /* a policy check failed */
pub const TPM2_RC_INTEGRITY: TPM2_RC = TPM2_RC_FMT1 + 0x01F; /* integrity check failed */
pub const TPM2_RC_TICKET: TPM2_RC = TPM2_RC_FMT1 + 0x020; /* invalid ticket */
pub const TPM2_RC_RESERVED_BITS: TPM2_RC = TPM2_RC_FMT1 + 0x021; /* reserved bits not set to zero as required */
pub const TPM2_RC_BAD_AUTH: TPM2_RC = TPM2_RC_FMT1 + 0x022; /* authorization failure without DA implications */
pub const TPM2_RC_EXPIRED: TPM2_RC = TPM2_RC_FMT1 + 0x023; /* the policy has expired */
pub const TPM2_RC_POLICY_CC: TPM2_RC = TPM2_RC_FMT1 + 0x024; /* the commandCode in the policy is not the commandCode of the command or the command code in a policy command references a command that is not implemented */
pub const TPM2_RC_BINDING: TPM2_RC = TPM2_RC_FMT1 + 0x025; /* public and sensitive portions of an object are not cryptographically bound */
pub const TPM2_RC_CURVE: TPM2_RC = TPM2_RC_FMT1 + 0x026; /* curve not supported */
pub const TPM2_RC_ECC_POINT: TPM2_RC = TPM2_RC_FMT1 + 0x027; /* point is not on the required curve. */
pub const TPM2_RC_WARN: TPM2_RC = 0x900; /* set for warning response codes */
pub const TPM2_RC_CONTEXT_GAP: TPM2_RC = TPM2_RC_WARN + 0x001; /* gap for context ID is too large */
pub const TPM2_RC_OBJECT_MEMORY: TPM2_RC = TPM2_RC_WARN + 0x002; /* out of memory for object contexts */
pub const TPM2_RC_SESSION_MEMORY: TPM2_RC = TPM2_RC_WARN + 0x003; /* out of memory for session contexts */
pub const TPM2_RC_MEMORY: TPM2_RC = TPM2_RC_WARN + 0x004; /* out of shared objectsession memory or need space for internal operations */
pub const TPM2_RC_SESSION_HANDLES: TPM2_RC = TPM2_RC_WARN + 0x005; /* out of session handles  a session must be flushed before a new session may be created */
pub const TPM2_RC_OBJECT_HANDLES: TPM2_RC = TPM2_RC_WARN + 0x006; /* out of object handles. The handle space for objects is depleted and a reboot is required. NOTE This cannot occur on the reference implementation. NOTE There is no reason why an implementation would implement a design that would deplete handle space. Platform specifications are encouraged to forbid it. */
pub const TPM2_RC_LOCALITY: TPM2_RC = TPM2_RC_WARN + 0x007; /* bad locality */
pub const TPM2_RC_YIELDED: TPM2_RC = TPM2_RC_WARN + 0x008; /* the TPM has suspended operation on the command forward progress was made and the command may be retried. See TPM 2.0 Part 1 Multitasking. NOTE This cannot occur on the reference implementation. */
pub const TPM2_RC_CANCELED: TPM2_RC = TPM2_RC_WARN + 0x009; /* the command was canceled */
pub const TPM2_RC_TESTING: TPM2_RC = TPM2_RC_WARN + 0x00A; /* TPM is performing selftests */
pub const TPM2_RC_REFERENCE_H0: TPM2_RC = TPM2_RC_WARN + 0x010; /* the 1st handle in the handle area references a transient object or session that is not loaded */
pub const TPM2_RC_REFERENCE_H1: TPM2_RC = TPM2_RC_WARN + 0x011; /* the 2nd handle in the handle area references a transient object or session that is not loaded */
pub const TPM2_RC_REFERENCE_H2: TPM2_RC = TPM2_RC_WARN + 0x012; /* the 3rd handle in the handle area references a transient object or session that is not loaded */
pub const TPM2_RC_REFERENCE_H3: TPM2_RC = TPM2_RC_WARN + 0x013; /* the 4th handle in the handle area references a transient object or session that is not loaded */
pub const TPM2_RC_REFERENCE_H4: TPM2_RC = TPM2_RC_WARN + 0x014; /* the 5th handle in the handle area references a transient object or session that is not loaded */
pub const TPM2_RC_REFERENCE_H5: TPM2_RC = TPM2_RC_WARN + 0x015; /* the 6th handle in the handle area references a transient object or session that is not loaded */
pub const TPM2_RC_REFERENCE_H6: TPM2_RC = TPM2_RC_WARN + 0x016; /* the 7th handle in the handle area references a transient object or session that is not loaded */
pub const TPM2_RC_REFERENCE_S0: TPM2_RC = TPM2_RC_WARN + 0x018; /* the 1st authorization session handle references a session that is not loaded */
pub const TPM2_RC_REFERENCE_S1: TPM2_RC = TPM2_RC_WARN + 0x019; /* the 2nd authorization session handle references a session that is not loaded */
pub const TPM2_RC_REFERENCE_S2: TPM2_RC = TPM2_RC_WARN + 0x01A; /* the 3rd authorization session handle references a session that is not loaded */
pub const TPM2_RC_REFERENCE_S3: TPM2_RC = TPM2_RC_WARN + 0x01B; /* the 4th authorization session handle references a session that is not loaded */
pub const TPM2_RC_REFERENCE_S4: TPM2_RC = TPM2_RC_WARN + 0x01C; /* the 5th session handle references a session that is not loaded */
pub const TPM2_RC_REFERENCE_S5: TPM2_RC = TPM2_RC_WARN + 0x01D; /* the 6th session handle references a session that is not loaded */
pub const TPM2_RC_REFERENCE_S6: TPM2_RC = TPM2_RC_WARN + 0x01E; /* the 7th authorization session handle references a session that is not loaded */
pub const TPM2_RC_NV_RATE: TPM2_RC = TPM2_RC_WARN + 0x020; /* the TPM is rate limiting accesses to prevent wearout of NV */
pub const TPM2_RC_LOCKOUT: TPM2_RC = TPM2_RC_WARN + 0x021; /* authorizations for objects subject to DA protection are not allowed at this time because the TPM is in DA lockout mode */
pub const TPM2_RC_RETRY: TPM2_RC = TPM2_RC_WARN + 0x022; /* the TPM was not able to start the command */
pub const TPM2_RC_NV_UNAVAILABLE: TPM2_RC = TPM2_RC_WARN + 0x023; /* the command may require writing of NV and NV is not current accessible */
pub const TPM2_RC_NOT_USED: TPM2_RC = TPM2_RC_WARN + 0x07F; /* this value is reserved and shall not be returned by the TPM */
pub const TPM2_RC_H: TPM2_RC = 0x000; /* add to a handle related error */
pub const TPM2_RC_P: TPM2_RC = 0x040; /* add to a parameter-related error */
pub const TPM2_RC_S: TPM2_RC = 0x800; /* add to a session-related error */
pub const TPM2_RC_1: TPM2_RC = 0x100; /* add to a parameter handle or session-related error */
pub const TPM2_RC_2: TPM2_RC = 0x200; /* add to a parameter handle or session-related error */
pub const TPM2_RC_3: TPM2_RC = 0x300; /* add to a parameter handle or session-related error */
pub const TPM2_RC_4: TPM2_RC = 0x400; /* add to a parameter handle or session-related error */
pub const TPM2_RC_5: TPM2_RC = 0x500; /* add to a parameter handle or session-related error */
pub const TPM2_RC_6: TPM2_RC = 0x600; /* add to a parameter handle or session-related error */
pub const TPM2_RC_7: TPM2_RC = 0x700; /* add to a parameter handle or session-related error */
pub const TPM2_RC_8: TPM2_RC = 0x800; /* add to a parameter-related error */
pub const TPM2_RC_9: TPM2_RC = 0x900; /* add to a parameter-related error */
pub const TPM2_RC_A: TPM2_RC = 0xA00; /* add to a parameter-related error */
pub const TPM2_RC_B: TPM2_RC = 0xB00; /* add to a parameter-related error */
pub const TPM2_RC_C: TPM2_RC = 0xC00; /* add to a parameter-related error */
pub const TPM2_RC_D: TPM2_RC = 0xD00; /* add to a parameter-related error */
pub const TPM2_RC_E: TPM2_RC = 0xE00; /* add to a parameter-related error */
pub const TPM2_RC_F: TPM2_RC = 0xF00; /* add to a parameter-related error */
pub const TPM2_RC_N_MASK: TPM2_RC = 0xF00; /* number mask */

pub const TPM2_CLOCK_COARSE_SLOWER: TPM2_CLOCK_ADJUST = -3; /* Slow the Clock update rate by one coarse adjustment step. */
pub const TPM2_CLOCK_MEDIUM_SLOWER: TPM2_CLOCK_ADJUST = -2; /* Slow the Clock update rate by one medium adjustment step. */
pub const TPM2_CLOCK_FINE_SLOWER: TPM2_CLOCK_ADJUST = -1; /* Slow the Clock update rate by one fine adjustment step. */
pub const TPM2_CLOCK_NO_CHANGE: TPM2_CLOCK_ADJUST = 0; /* No change to the Clock update rate. */
pub const TPM2_CLOCK_FINE_FASTER: TPM2_CLOCK_ADJUST = 1; /* Speed the Clock update rate by one fine adjustment step. */
pub const TPM2_CLOCK_MEDIUM_FASTER: TPM2_CLOCK_ADJUST = 2; /* Speed the Clock update rate by one medium adjustment step. */
pub const TPM2_CLOCK_COARSE_FASTER: TPM2_CLOCK_ADJUST = 3; /* Speed the Clock update rate by one coarse adjustment step. */

pub const TPM2_EO_EQ: TPM2_EO = 0x0000; /* A  B */
pub const TPM2_EO_NEQ: TPM2_EO = 0x0001; /* A  B */
pub const TPM2_EO_SIGNED_GT: TPM2_EO = 0x0002; /* A > B signed */
pub const TPM2_EO_UNSIGNED_GT: TPM2_EO = 0x0003; /* A > B unsigned */
pub const TPM2_EO_SIGNED_LT: TPM2_EO = 0x0004; /* A < B signed */
pub const TPM2_EO_UNSIGNED_LT: TPM2_EO = 0x0005; /* A < B unsigned */
pub const TPM2_EO_SIGNED_GE: TPM2_EO = 0x0006; /* A  B signed */
pub const TPM2_EO_UNSIGNED_GE: TPM2_EO = 0x0007; /* A  B unsigned */
pub const TPM2_EO_SIGNED_LE: TPM2_EO = 0x0008; /* A  B signed */
pub const TPM2_EO_UNSIGNED_LE: TPM2_EO = 0x0009; /* A  B unsigned */
pub const TPM2_EO_BITSET: TPM2_EO = 0x000A; /* All bits SET in B are SET in A. ABB */
pub const TPM2_EO_BITCLEAR: TPM2_EO = 0x000B; /* All bits SET in B are CLEAR in A. AB0 */

pub const TPM2_ST_RSP_COMMAND: TPM2_ST = 0x00C4; /* Tag value for a response used when there is an error in the tag. This is also the value returned from a TPM 1.2 when an error occurs. This value is used in this specification because an error in the command tag may prevent determination of the family. When this tag is used in the response the response code will be TPM2_RC_BAD_TAG 0 1E16 which has the same numeric value as the TPM 1.2 response code for TPM_BADTAG. NOTE In a previously published version of this specification TPM2_RC_BAD_TAG was incorrectly assigned a value of 0x030 instead of 30 0x01e. Some implementations my return the old value instead of the new value. */
pub const TPM2_ST_NULL: TPM2_ST = 0x8000; /* no structure type specified */
pub const TPM2_ST_NO_SESSIONS: TPM2_ST = 0x8001; /* tag value for a command response for a command defined in this specification indicating that the command response has no attached sessions and no authorizationSizeparameterSize value is present. If the responseCode from the TPM is not TPM2_RC_SUCCESS then the response tag shall have this value. */
pub const TPM2_ST_SESSIONS: TPM2_ST = 0x8002; /* tag value for a command response for a command defined in this specification indicating that the command response has one or more attached sessions and the authorizationSizeparameterSize field is present */
pub const TPM2_ST_RESERVED1: TPM2_ST = 0x8003; /* When used between application software and the TPM resource manager, this tag indicates that the command has no sessions and the handles are using the Name format rather than the 32-bit handle format. NOTE 1 The response to application software will have a tag of TPM2_ST_NO_SESSIONS. Between the TRM and TPM, this tag would occur in a response from a TPM that overlaps the tag parameter of a request with the tag parameter of a response when the response has no associated sessions. NOTE 2 This tag is not used by all TPM or TRM implementations. */
pub const TPM2_ST_RESERVED2: TPM2_ST = 0x8004; /* When used between application software and the TPM resource manager. This tag indicates that the command has sessions and the handles are using the Name format rather than the 32-bit handle format. NOTE 1 If the command completes successfully the response to application software will have a tag of TPM2_ST_SESSIONS. Between the TRM and TPM would occur in a response from a TPM that overlaps the tag parameter of a request with the tag parameter of a response when the response has authorization sessions. NOTE 2 This tag is not used by all TPM or TRM implementations. */
pub const TPM2_ST_ATTEST_NV: TPM2_ST = 0x8014; /* tag for an attestation structure */
pub const TPM2_ST_ATTEST_COMMAND_AUDIT: TPM2_ST = 0x8015; /* tag for an attestation structure */
pub const TPM2_ST_ATTEST_SESSION_AUDIT: TPM2_ST = 0x8016; /* tag for an attestation structure */
pub const TPM2_ST_ATTEST_CERTIFY: TPM2_ST = 0x8017; /* tag for an attestation structure */
pub const TPM2_ST_ATTEST_QUOTE: TPM2_ST = 0x8018; /* tag for an attestation structure */
pub const TPM2_ST_ATTEST_TIME: TPM2_ST = 0x8019; /* tag for an attestation structure */
pub const TPM2_ST_ATTEST_CREATION: TPM2_ST = 0x801A; /* tag for an attestation structure */
pub const TPM2_ST_RESERVED3: TPM2_ST = 0x801B; /* do not use . NOTE This was previously assigned to TPM2_ST_ATTEST_NV. The tag is changed because the structure has changed */
pub const TPM2_ST_ATTEST_NV_DIGEST: TPM2_ST = 0x801c; /* tag for an attestation structure */
pub const TPM2_ST_CREATION: TPM2_ST = 0x8021; /* tag for a ticket type */
pub const TPM2_ST_VERIFIED: TPM2_ST = 0x8022; /* tag for a ticket type */
pub const TPM2_ST_AUTH_SECRET: TPM2_ST = 0x8023; /* tag for a ticket type */
pub const TPM2_ST_HASHCHECK: TPM2_ST = 0x8024; /* tag for a ticket type */
pub const TPM2_ST_AUTH_SIGNED: TPM2_ST = 0x8025; /* tag for a ticket type */
pub const TPM2_ST_FU_MANIFEST: TPM2_ST = 0x8029; /* tag for a structure describing a Field Upgrade Policy */

pub const TPM2_SU_CLEAR: TPM2_SU = 0x0000; /* On TPM2_Shutdown indicates that the TPM should prepare for loss of power and save state required for an orderly startup TPM Reset. On TPM2_Startup indicates that the TPM should perform TPM Reset or TPM Restart */
pub const TPM2_SU_STATE: TPM2_SU = 0x0001; /* On TPM2_Shutdown indicates that the TPM should prepare for loss of power and save state required for an orderly startup. TPM Restart or TPM Resume on TPM2_Startup indicates that the TPM should restore the state saved by TPM2_Shutdown TPM2_SU_STATE */

pub const TPM2_SE_HMAC: TPM2_SE = 0x00;
pub const TPM2_SE_POLICY: TPM2_SE = 0x01;
pub const TPM2_SE_TRIAL: TPM2_SE = 0x03; /* The policy session is being used to compute the policyHash and not for command authorization.This setting modifies some policy commands and prevents session from being used to authorize a command. */

pub const TPM2_CAP_FIRST: TPM2_CAP = 0x00000000;
pub const TPM2_CAP_ALGS: TPM2_CAP = 0x00000000; /* TPM2_ALG_ID1 */
pub const TPM2_CAP_HANDLES: TPM2_CAP = 0x00000001; /* TPM2_HANDLE */
pub const TPM2_CAP_COMMANDS: TPM2_CAP = 0x00000002; /* TPM2_CC */
pub const TPM2_CAP_PP_COMMANDS: TPM2_CAP = 0x00000003; /* TPM2_CC */
pub const TPM2_CAP_AUDIT_COMMANDS: TPM2_CAP = 0x00000004; /* TPM2_CC */
pub const TPM2_CAP_PCRS: TPM2_CAP = 0x00000005; /* reserved */
pub const TPM2_CAP_TPM_PROPERTIES: TPM2_CAP = 0x00000006; /* TPM2_PT */
pub const TPM2_CAP_PCR_PROPERTIES: TPM2_CAP = 0x00000007; /* TPM2_PT_PCR */
pub const TPM2_CAP_ECC_CURVES: TPM2_CAP = 0x00000008; /* TPM2_ECC_CURVE */
pub const TPM2_CAP_AUTH_POLICIES: TPM2_CAP = 0x00000009; /* TPM2_HANDLE */
pub const TPM2_CAP_ACT: TPM2_CAP = 0x0000000A; /* TPM2_HANDLE */
pub const TPM2_CAP_LAST: TPM2_CAP = 0x0000000A;
pub const TPM2_CAP_VENDOR_PROPERTY: TPM2_CAP = 0x00000100; /* manufacturer specific */

pub const TPM2_NT_ORDINARY: TPM2_NT = 0x0; /* Ordinary – contains data that is opaque to the TPM that can only be modified using TPM2_NV_Write(). */
pub const TPM2_NT_COUNTER: TPM2_NT = 0x1; /* Counter – contains an 8-octet value that is to be used as a counter and can only be modified with TPM2_NV_Increment() */
pub const TPM2_NT_BITS: TPM2_NT = 0x2; /* Bit Field – contains an 8-octet value to be used as a bit field and can only be modified with TPM2_NV_SetBits(). */
pub const TPM2_NT_EXTEND: TPM2_NT = 0x4; /* Extend – contains a digest-sized value used like a PCR. The Index can only be modified using TPM2_NV_Extend(). The extend will use the nameAlg of the Index. */
pub const TPM2_NT_PIN_FAIL: TPM2_NT = 0x8; /* PIN Fail - contains pinCount that increments on a PIN authorization failure and a pinLimit. */
pub const TPM2_NT_PIN_PASS: TPM2_NT = 0x9; /* PIN Pass - contains pinCount that increments on a PIN authorization success and a pinLimit. */

pub const TPM2_PT_NONE: TPM2_PT = 0x00000000; /* indicates no property type */
pub const TPM2_PT_GROUP: TPM2_PT = 0x00000100; /* The number of properties in each group. NOTE The first group with any properties is group 1 TPM2_PT_GROUP * 1. Group 0 is reserved. */
pub const TPM2_PT_FIXED: TPM2_PT = TPM2_PT_GROUP * 1; /* the group of fixed properties returned as TPMS_TAGGED_PROPERTY. The values in this group are only changed due to a firmware change in the TPM. */
pub const TPM2_PT_FAMILY_INDICATOR: TPM2_PT = TPM2_PT_FIXED + 0; /* a 4-octet character string containing the TPM Family value TPM2_SPEC_FAMILY */
pub const TPM2_PT_LEVEL: TPM2_PT = TPM2_PT_FIXED + 1; /* the level of the specification. NOTE 1 For this specification the level is zero. NOTE 2 The level is on the title page of the specification. */
pub const TPM2_PT_REVISION: TPM2_PT = TPM2_PT_FIXED + 2; /* the specification Revision times 100. EXAMPLE Revision 01.01 would have a value of 101. NOTE The Revision value is on the title page of the specification. */
pub const TPM2_PT_DAY_OF_YEAR: TPM2_PT = TPM2_PT_FIXED + 3; /* the specification day of year using TCG calendar. EXAMPLE November 15 2010 has a day of year value of 319 00 00 01 3F16. NOTE The specification date is on the title page of the specification. */
pub const TPM2_PT_YEAR: TPM2_PT = TPM2_PT_FIXED + 4; /* the specification year using the CE. EXAMPLE The year 2010 has a value of 00 00 07 DA16. NOTE The specification date is on the title page of the specification. */
pub const TPM2_PT_MANUFACTURER: TPM2_PT = TPM2_PT_FIXED + 5; /* the vendor ID unique to each TPM manufacturer */
pub const TPM2_PT_VENDOR_STRING_1: TPM2_PT = TPM2_PT_FIXED + 6; /* the first four characters of the vendor ID string. NOTE When the vendor string is fewer than 16 octets the additional property values do not have to be present. A vendor string of 4 octets can be represented in one 32-bit value and no null terminating character is required. */
pub const TPM2_PT_VENDOR_STRING_2: TPM2_PT = TPM2_PT_FIXED + 7; /* the second four characters of the vendor ID string */
pub const TPM2_PT_VENDOR_STRING_3: TPM2_PT = TPM2_PT_FIXED + 8; /* the third four characters of the vendor ID string */
pub const TPM2_PT_VENDOR_STRING_4: TPM2_PT = TPM2_PT_FIXED + 9; /* the fourth four characters of the vendor ID string */
pub const TPM2_PT_VENDOR_TPM_TYPE: TPM2_PT = TPM2_PT_FIXED + 10; /* vendor defined value indicating the TPM model */
pub const TPM2_PT_FIRMWARE_VERSION_1: TPM2_PT = TPM2_PT_FIXED + 11; /* the most significant 32 bits of a TPM vendor-specific value indicating the version number of the firmware. See 10.12.2 and 10.12.8. */
pub const TPM2_PT_FIRMWARE_VERSION_2: TPM2_PT = TPM2_PT_FIXED + 12; /* the least significant 32 bits of a TPM vendor-specific value indicating the version number of the firmware. See 10.12.2 and 10.12.8. */
pub const TPM2_PT_INPUT_BUFFER: TPM2_PT = TPM2_PT_FIXED + 13; /* the maximum size of a parameter typically a TPM2B_MAX_BUFFER */
pub const TPM2_PT_HR_TRANSIENT_MIN: TPM2_PT = TPM2_PT_FIXED + 14; /* the minimum number of transient objects that can be held in TPM RAM. NOTE This minimum shall be no less than the minimum value required by the platforms-pecific specification to which the TPM is built. */
pub const TPM2_PT_HR_PERSISTENT_MIN: TPM2_PT = TPM2_PT_FIXED + 15; /* the minimum number of persistent objects that can be held in TPM NV memory. NOTE This minimum shall be no less than the minimum value required by the platform-specific specification to which the TPM is built. */
pub const TPM2_PT_HR_LOADED_MIN: TPM2_PT = TPM2_PT_FIXED + 16; /* the minimum number of authorization sessions that can be held in TPM RAM . NOTE This minimum shall be no less than the minimum value required by the platform-specific specification to which the TPM is built. */
pub const TPM2_PT_ACTIVE_SESSIONS_MAX: TPM2_PT = TPM2_PT_FIXED + 17; /* the number of authorization sessions that may be active at a time. A session is active when it has a context associated with its handle. The context may either be in TPM RAM or be context saved. NOTE This value shall be no less than the minimum value required by the platform-specific specification to which the TPM is built. */
pub const TPM2_PT_PCR_COUNT: TPM2_PT = TPM2_PT_FIXED + 18; /* the number of PCR implemented. NOTE This number is determined by the defined attributes not the number of PCR that are populated. */
pub const TPM2_PT_PCR_SELECT_MIN: TPM2_PT = TPM2_PT_FIXED + 19; /* the minimum number of octets in a TPMS_PCR_SELECT.sizeofSelect. NOTE This value is not determined by the number of PCR implemented but by the number of PCR required by the platform-specific specification with which the TPM is compliant or by the implementer if not adhering to a platform-specific specification. */
pub const TPM2_PT_CONTEXT_GAP_MAX: TPM2_PT = TPM2_PT_FIXED + 20; /* the maximum allowed difference unsigned between the contextID values of two saved session contexts. This value shall be 2n1 where n is at least 16. */
pub const TPM2_PT_NV_COUNTERS_MAX: TPM2_PT = TPM2_PT_FIXED + 22; /* the maximum number of NV Indexes that are allowed to have the TPM2_NT_COUNTER attribute. NOTE It is allowed for this value to be larger than the number of NV Indexes that can be defined. This would be indicative of a TPM implementation that did not use different implementation technology for different NV Index types. */
pub const TPM2_PT_NV_INDEX_MAX: TPM2_PT = TPM2_PT_FIXED + 23; /* the maximum size of an NV Index data area */
pub const TPM2_PT_MEMORY: TPM2_PT = TPM2_PT_FIXED + 24; /* a TPMA_MEMORY indicating the memory management method for the TPM */
pub const TPM2_PT_CLOCK_UPDATE: TPM2_PT = TPM2_PT_FIXED + 25; /* interval in milliseconds between updates to the copy of TPMS_CLOCK_INFO.clock in NV */
pub const TPM2_PT_CONTEXT_HASH: TPM2_PT = TPM2_PT_FIXED + 26; /* the algorithm used for the integrity HMAC on saved contexts and for hashing the fuData of TPM2_FirmwareRead */
pub const TPM2_PT_CONTEXT_SYM: TPM2_PT = TPM2_PT_FIXED + 27; /* TPM2_ALG_ID the algorithm used for encryption of saved contexts */
pub const TPM2_PT_CONTEXT_SYM_SIZE: TPM2_PT = TPM2_PT_FIXED + 28; /* TPM2_KEY_BITS the size of the key used for encryption of saved contexts */
pub const TPM2_PT_ORDERLY_COUNT: TPM2_PT = TPM2_PT_FIXED + 29; /* the modulus 1 of the count for NV update of an orderly counter. The returned value is MAX_ORDERLY_COUNT. This will have a value of 2N  1 where 1  N  32. NOTE An orderly counter is an NV Index with an TPM2_NT of TPM_NV_COUNTER and TPMA_NV_ORDERLY SET. NOTE When the low-order bits of a counter equal this value an NV write occurs on the next increment. */
pub const TPM2_PT_MAX_COMMAND_SIZE: TPM2_PT = TPM2_PT_FIXED + 30; /* the maximum value for commandSize in a command */
pub const TPM2_PT_MAX_RESPONSE_SIZE: TPM2_PT = TPM2_PT_FIXED + 31; /* the maximum value for responseSize in a response */
pub const TPM2_PT_MAX_DIGEST: TPM2_PT = TPM2_PT_FIXED + 32; /* the maximum size of a digest that can be produced by the TPM */
pub const TPM2_PT_MAX_OBJECT_CONTEXT: TPM2_PT = TPM2_PT_FIXED + 33; /* the maximum size of an object context that will be returned by TPM2_ContextSave */
pub const TPM2_PT_MAX_SESSION_CONTEXT: TPM2_PT = TPM2_PT_FIXED + 34; /* the maximum size of a session context that will be returned by TPM2_ContextSave */
pub const TPM2_PT_PS_FAMILY_INDICATOR: TPM2_PT = TPM2_PT_FIXED + 35; /* platform-specific family. A TPM2_PS value. See Table 25. NOTE The platform-specific values for the TPM2_PT_PS parameters are in the relevant platform-specific specification. In the reference implementation all of these values are 0. */
pub const TPM2_PT_PS_LEVEL: TPM2_PT = TPM2_PT_FIXED + 36; /* the level of the platform-specific specification */
pub const TPM2_PT_PS_REVISION: TPM2_PT = TPM2_PT_FIXED + 37; /* the specification Revision times 100 for the platform-specific specification */
pub const TPM2_PT_PS_DAY_OF_YEAR: TPM2_PT = TPM2_PT_FIXED + 38; /* the platform-specific specification day of year using TCG calendar */
pub const TPM2_PT_PS_YEAR: TPM2_PT = TPM2_PT_FIXED + 39; /* the platform-specific specification year using the CE */
pub const TPM2_PT_SPLIT_MAX: TPM2_PT = TPM2_PT_FIXED + 40; /* the number of split signing operations supported by the TPM */
pub const TPM2_PT_TOTAL_COMMANDS: TPM2_PT = TPM2_PT_FIXED + 41; /* total number of commands implemented in the TPM */
pub const TPM2_PT_LIBRARY_COMMANDS: TPM2_PT = TPM2_PT_FIXED + 42; /* number of commands from the TPM library that are implemented */
pub const TPM2_PT_VENDOR_COMMANDS: TPM2_PT = TPM2_PT_FIXED + 43; /* number of vendor commands that are implemented */
pub const TPM2_PT_NV_BUFFER_MAX: TPM2_PT = TPM2_PT_FIXED + 44; /* the maximum data size in one NV write command */
pub const TPM2_PT_MODES: TPM2_PT = TPM2_PT_FIXED + 45; /* a TPMA_MODES value indicating that the TPM is designed for these modes. */
pub const TPM2_PT_MAX_CAP_BUFFER: TPM2_PT = TPM2_PT_FIXED + 46; /* the maximum size of a TPMS_CAPABILITY_DATA structure returned in TPM2_GetCapability(). */
pub const TPM2_PT_VAR: TPM2_PT = TPM2_PT_GROUP * 2; /* the group of variable properties returned as TPMS_TAGGED_PROPERTY. The properties in this group change because of a Protected Capability other than a firmware update. The values are not necessarily persistent across all power transitions. */
pub const TPM2_PT_PERMANENT: TPM2_PT = TPM2_PT_VAR + 0; /* TPMA_PERMANENT */
pub const TPM2_PT_STARTUP_CLEAR: TPM2_PT = TPM2_PT_VAR + 1; /* TPMA_STARTUP_CLEAR */
pub const TPM2_PT_HR_NV_INDEX: TPM2_PT = TPM2_PT_VAR + 2; /* the number of NV Indexes currently defined */
pub const TPM2_PT_HR_LOADED: TPM2_PT = TPM2_PT_VAR + 3; /* the number of authorization sessions currently loaded into TPM RAM */
pub const TPM2_PT_HR_LOADED_AVAIL: TPM2_PT = TPM2_PT_VAR + 4; /* the number of additional authorization sessions of any type that could be loaded into TPM RAM. This value is an estimate. If this value is at least 1 then at least one authorization session of any type may be loaded. Any command that changes the RAM memory allocation can make this estimate invalid. NOTE A valid implementation may return 1 even if more than one authorization session would fit into RAM. */
pub const TPM2_PT_HR_ACTIVE: TPM2_PT = TPM2_PT_VAR + 5; /* the number of active authorization sessions currently being tracked by the TPMThis is the sum of the loaded and saved sessions. */
pub const TPM2_PT_HR_ACTIVE_AVAIL: TPM2_PT = TPM2_PT_VAR + 6; /* the number of additional authorization sessions of any type that could be created. This value is an estimate. If this value is at least 1 then at least one authorization session of any type may be created. Any command that changes the RAM memory allocation can make this estimate invalid. NOTE A valid implementation may return 1 even if more than one authorization session could be created. */
pub const TPM2_PT_HR_TRANSIENT_AVAIL: TPM2_PT = TPM2_PT_VAR + 7; /* estimate of the number of additional transient objects that could be loaded into TPM RAM. This value is an estimate. If this value is at least 1 then at least one object of any type may be loaded. Any command that changes the memory allocation can make this estimate invalid. NOTE A valid implementation may return 1 even if more than one transient object would fit into RAM. */
pub const TPM2_PT_HR_PERSISTENT: TPM2_PT = TPM2_PT_VAR + 8; /* the number of persistent objects currently loaded into TPM NV memory */
pub const TPM2_PT_HR_PERSISTENT_AVAIL: TPM2_PT = TPM2_PT_VAR + 9; /* the number of additional persistent objects that could be loaded into NV memory. This value is an estimate. If this value is at least 1 then at least one object of any type may be made persistent. Any command that changes the NV memory allocation can make this estimate invalid. NOTE A valid implementation may return 1 even if more than one persistent object would fit into NV memory. */
pub const TPM2_PT_NV_COUNTERS: TPM2_PT = TPM2_PT_VAR + 10; /* the number of defined NV Indexes that have NV the TPM2_NT_COUNTER attribute */
pub const TPM2_PT_NV_COUNTERS_AVAIL: TPM2_PT = TPM2_PT_VAR + 11; /* the number of additional NV Indexes that can be defined with their TPM2_NT of TPM_NV_COUNTER and the TPMA_NV_ORDERLY attribute SET. This value is an estimate. If this value is at least 1 then at least one NV Index may be created with a TPM2_NT of TPM_NV_COUNTER and the TPMA_NV_ORDERLY attributes. Any command that changes the NV memory allocation can make this estimate invalid. NOTE A valid implementation may return 1 even if more than one NV counter could be defined. */
pub const TPM2_PT_ALGORITHM_SET: TPM2_PT = TPM2_PT_VAR + 12; /* code that limits the algorithms that may be used with the TPM */
pub const TPM2_PT_LOADED_CURVES: TPM2_PT = TPM2_PT_VAR + 13; /* the number of loaded ECC curves */
pub const TPM2_PT_LOCKOUT_COUNTER: TPM2_PT = TPM2_PT_VAR + 14; /* the current value of the lockout counter failedTries */
pub const TPM2_PT_MAX_AUTH_FAIL: TPM2_PT = TPM2_PT_VAR + 15; /* the number of authorization failures before DA lockout is invoked */
pub const TPM2_PT_LOCKOUT_INTERVAL: TPM2_PT = TPM2_PT_VAR + 16; /* the number of seconds before the value reported by TPM2_PT_LOCKOUT_COUNTER is decremented */
pub const TPM2_PT_LOCKOUT_RECOVERY: TPM2_PT = TPM2_PT_VAR + 17; /* the number of seconds after a lockoutAuth failure before use of lockoutAuth may be attempted again */
pub const TPM2_PT_NV_WRITE_RECOVERY: TPM2_PT = TPM2_PT_VAR + 18; /* number of milliseconds before the TPM will accept another command that will modify NVThis value is an approximation and may go up or down over time. */
pub const TPM2_PT_AUDIT_COUNTER_0: TPM2_PT = TPM2_PT_VAR + 19; /* the high-order 32 bits of the command audit counter */
pub const TPM2_PT_AUDIT_COUNTER_1: TPM2_PT = TPM2_PT_VAR + 20; /* the low-order 32 bits of the command audit counter */

pub const TPM2_PT_TPM2_PCR_FIRST: TPM2_PT_PCR = 0x00000000; /* bottom of the range of TPM2_PT_PCR properties */
pub const TPM2_PT_PCR_SAVE: TPM2_PT_PCR = 0x00000000; /* a SET bit in the TPMS_PCR_SELECT indicates that the PCR is saved and restored by TPM2_SU_STATE */
pub const TPM2_PT_PCR_EXTEND_L0: TPM2_PT_PCR = 0x00000001; /* a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be extended from locality 0This property is only present if a locality other than 0 is implemented. */
pub const TPM2_PT_PCR_RESET_L0: TPM2_PT_PCR = 0x00000002; /* a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be reset by TPM2_PCR_Reset from locality 0 */
pub const TPM2_PT_PCR_EXTEND_L1: TPM2_PT_PCR = 0x00000003; /* a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be extended from locality 1 This property is only present if locality 1 is implemented. */
pub const TPM2_PT_PCR_RESET_L1: TPM2_PT_PCR = 0x00000004; /* a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be reset by TPM2_PCR_Reset from locality 1This property is only present if locality 1 is implemented. */
pub const TPM2_PT_PCR_EXTEND_L2: TPM2_PT_PCR = 0x00000005; /* a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be extended from locality 2 This property is only present if localities 1 and 2 are implemented. */
pub const TPM2_PT_PCR_RESET_L2: TPM2_PT_PCR = 0x00000006; /* a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be reset by TPM2_PCR_Reset from locality 2This property is only present if localities 1 and 2 are implemented. */
pub const TPM2_PT_PCR_EXTEND_L3: TPM2_PT_PCR = 0x00000007; /* a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be extended from locality 3This property is only present if localities 1 2 and 3 are implemented. */
pub const TPM2_PT_PCR_RESET_L3: TPM2_PT_PCR = 0x00000008; /* a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be reset by TPM2_PCR_Reset from locality 3This property is only present if localities 1 2 and 3 are implemented. */
pub const TPM2_PT_PCR_EXTEND_L4: TPM2_PT_PCR = 0x00000009; /* a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be extended from locality 4This property is only present if localities 1 2 3 and 4 are implemented. */
pub const TPM2_PT_PCR_RESET_L4: TPM2_PT_PCR = 0x0000000A; /* a SET bit in the TPMS_PCR_SELECT indicates that the PCR may be reset by TPM2_PCR_Reset from locality 4This property is only present if localities 1 2 3 and 4 are implemented. */

pub const TPM2_PT_PCR_NO_INCREMENT: TPM2_PT_PCR = 0x00000011; /* a SET bit in the TPMS_PCR_SELECT indicates that modifications to this PCR reset or Extend will not increment the pcrUpdateCounter */
pub const TPM2_PT_PCR_DRTM_RESET: TPM2_PT_PCR = 0x00000012; /* a SET bit in the TPMS_PCR_SELECT indicates that the PCR is reset by a DRTM event. These PCR are reset to 1 on TPM2_Startup and reset to 0 on a _TPM_Hash_End event following a _TPM_Hash_Start event. */
pub const TPM2_PT_PCR_POLICY: TPM2_PT_PCR = 0x00000013; /* a SET bit in the TPMS_PCR_SELECT indicates that the PCR is controlled by policy. This property is only present if the TPM supports policy control of a PCR. */
pub const TPM2_PT_PCR_AUTH: TPM2_PT_PCR = 0x00000014; /* a SET bit in the TPMS_PCR_SELECT indicates that the PCR is controlled by an authorization value. This property is only present if the TPM supports authorization control of a PCR. */
pub const TPM2_PT_TPM2_PCR_LAST: TPM2_PT_PCR = 0x00000014; /* top of the range of TPM2_PT_PCR properties of the implementation. If the TPM receives a request for a PCR property with a value larger than this the TPM will return a zero length list and set the moreData parameter to NO. NOTE This is an implementation-specific value. The value shown reflects the reference code implementation. */

pub const TPM2_PS_MAIN: TPM2_PS = 0x00000000; /* not platform-specific */
pub const TPM2_PS_PC: TPM2_PS = 0x00000001; /* PC Client */
pub const TPM2_PS_PDA: TPM2_PS = 0x00000002; /* PDA includes all mobile devices that are not specifically cell phones */
pub const TPM2_PS_CELL_PHONE: TPM2_PS = 0x00000003; /* Cell Phone */
pub const TPM2_PS_SERVER: TPM2_PS = 0x00000004; /* Server WG */
pub const TPM2_PS_PERIPHERAL: TPM2_PS = 0x00000005; /* Peripheral WG */
pub const TPM2_PS_TSS: TPM2_PS = 0x00000006; /* TSS WG */
pub const TPM2_PS_STORAGE: TPM2_PS = 0x00000007; /* Storage WG */
pub const TPM2_PS_AUTHENTICATION: TPM2_PS = 0x00000008; /* Authentication WG */
pub const TPM2_PS_EMBEDDED: TPM2_PS = 0x00000009; /* Embedded WG */
pub const TPM2_PS_HARDCOPY: TPM2_PS = 0x0000000A; /* Hardcopy WG */
pub const TPM2_PS_INFRASTRUCTURE: TPM2_PS = 0x0000000B; /* Infrastructure WG */
pub const TPM2_PS_VIRTUALIZATION: TPM2_PS = 0x0000000C; /* Virtualization WG */
pub const TPM2_PS_TNC: TPM2_PS = 0x0000000D; /* Trusted Network Connect WG */
pub const TPM2_PS_MULTI_TENANT: TPM2_PS = 0x0000000E; /* Multi-tenant WG */
pub const TPM2_PS_TC: TPM2_PS = 0x0000000F; /* Technical Committee */

pub const TPM2_HT_PCR: TPM2_HT = 0x00; /* PCR  consecutive numbers starting at 0 that reference the PCR registers. A platform-specific specification will set the minimum number of PCR and an implementation may have more. */
pub const TPM2_HT_NV_INDEX: TPM2_HT = 0x01; /* NV Index  assigned by the caller */
pub const TPM2_HT_HMAC_SESSION: TPM2_HT = 0x02; /* HMAC Authorization Session  assigned by the TPM when the session is created */
pub const TPM2_HT_LOADED_SESSION: TPM2_HT = 0x02; /* Loaded Authorization Session  used only in the context of TPM2_GetCapability. This type references both loaded HMAC and loaded policy authorization sessions. */
pub const TPM2_HT_POLICY_SESSION: TPM2_HT = 0x03; /* Policy Authorization Session  assigned by the TPM when the session is created */
pub const TPM2_HT_SAVED_SESSION: TPM2_HT = 0x03; /* Saved Authorization Session  used only in the context of TPM2_GetCapability. This type references saved authorization session contexts for which the TPM is maintaining tracking information. */
pub const TPM2_HT_PERMANENT: TPM2_HT = 0x40; /* Permanent Values  assigned by this specification in */
pub const TPM2_HT_TRANSIENT: TPM2_HT = 0x80; /* Transient Objects  assigned by the TPM when an object is loaded into transient object memory or when a persistent object is converted to a transient object */
pub const TPM2_HT_PERSISTENT: TPM2_HT = 0x81; /* Persistent Objects  assigned by the TPM when a loaded transient object is made persistent */
pub const TPM2_HT_AC: TPM2_HT = 0x90; /* Attached Component – handle for an Attached Component. */

pub const TPM2_RH_FIRST: TPM2_RH = 0x40000000; /* R */
pub const TPM2_RH_SRK: TPM2_RH = 0x40000000; /* R */
pub const TPM2_RH_OWNER: TPM2_RH = 0x40000001; /* K A P */
pub const TPM2_RH_REVOKE: TPM2_RH = 0x40000002; /* R */
pub const TPM2_RH_TRANSPORT: TPM2_RH = 0x40000003; /* R */
pub const TPM2_RH_OPERATOR: TPM2_RH = 0x40000004; /* R */
pub const TPM2_RH_ADMIN: TPM2_RH = 0x40000005; /* R */
pub const TPM2_RH_EK: TPM2_RH = 0x40000006; /* R */
pub const TPM2_RH_NULL: TPM2_RH = 0x40000007; /* K A P */
pub const TPM2_RH_UNASSIGNED: TPM2_RH = 0x40000008; /* R */
pub const TPM2_RS_PW: TPM2_RH = 0x40000009; /* S */
pub const TPM2_RH_LOCKOUT: TPM2_RH = 0x4000000A; /* A */
pub const TPM2_RH_ENDORSEMENT: TPM2_RH = 0x4000000B; /* K A P */
pub const TPM2_RH_PLATFORM: TPM2_RH = 0x4000000C; /* K A P */
pub const TPM2_RH_PLATFORM_NV: TPM2_RH = 0x4000000D; /* C */
pub const TPM2_RH_AUTH_00: TPM2_RH = 0x40000010; /* A */
pub const TPM2_RH_AUTH_FF: TPM2_RH = 0x4000010F; /* A */
pub const TPM2_RH_ACT_0: TPM2_RH = 0x40000110; /* A P */
pub const TPM2_RH_ACT_F: TPM2_RH = 0x4000011F; /* A P */
pub const TPM2_RH_LAST: TPM2_RH = 0x4000010F; /* R */

pub const TPM2_HR_HANDLE_MASK: TPM2_HC = 0x00FFFFFF; /* to mask off the HR */
pub const TPM2_HR_RANGE_MASK: TPM2_HC = 0xFF000000; /* to mask off the variable part */
pub const TPM2_HR_SHIFT: TPM2_HC = 24;

pub const TPM2_HR_PCR: TPM2_HC = (TPM2_HT_PCR as TPM2_HC) << TPM2_HR_SHIFT;
pub const TPM2_HR_HMAC_SESSION: TPM2_HC = (TPM2_HT_HMAC_SESSION as TPM2_HC) << TPM2_HR_SHIFT;
pub const TPM2_HR_POLICY_SESSION: TPM2_HC = (TPM2_HT_POLICY_SESSION as TPM2_HC) << TPM2_HR_SHIFT;
pub const TPM2_HR_TRANSIENT: TPM2_HC = (TPM2_HT_TRANSIENT as TPM2_HC) << TPM2_HR_SHIFT;
pub const TPM2_HR_PERSISTENT: TPM2_HC = (TPM2_HT_PERSISTENT as TPM2_HC) << TPM2_HR_SHIFT;
pub const TPM2_HR_NV_INDEX: TPM2_HC = (TPM2_HT_NV_INDEX as TPM2_HC) << TPM2_HR_SHIFT;
pub const TPM2_HR_PERMANENT: TPM2_HC = (TPM2_HT_PERMANENT as TPM2_HC) << TPM2_HR_SHIFT;
pub const TPM2_PCR_FIRST: TPM2_HC = TPM2_HR_PCR + 0; /* first PCR */
pub const TPM2_PCR_LAST: TPM2_HC = TPM2_PCR_FIRST + TPM2_MAX_PCRS - 1; /* last PCR */
pub const TPM2_HMAC_SESSION_FIRST: TPM2_HC = TPM2_HR_HMAC_SESSION + 0; /* first HMAC session */
pub const TPM2_HMAC_SESSION_LAST: TPM2_HC = TPM2_HMAC_SESSION_FIRST + 0x00fffffe; /* last HMAC session */
pub const TPM2_LOADED_SESSION_FIRST: TPM2_HC = TPM2_HMAC_SESSION_FIRST; /* used in GetCapability */
pub const TPM2_LOADED_SESSION_LAST: TPM2_HC = TPM2_HMAC_SESSION_LAST; /* used in GetCapability */
pub const TPM2_POLICY_SESSION_FIRST: TPM2_HC = TPM2_HR_POLICY_SESSION + 0; /* first policy session */
pub const TPM2_POLICY_SESSION_LAST: TPM2_HC = TPM2_POLICY_SESSION_FIRST + 0x00fffffe; /* last policy session */
pub const TPM2_TRANSIENT_FIRST: TPM2_HC = TPM2_HR_TRANSIENT + 0; /* first transient object */
pub const TPM2_ACTIVE_SESSION_FIRST: TPM2_HC = TPM2_POLICY_SESSION_FIRST; /* used in GetCapability */
pub const TPM2_ACTIVE_SESSION_LAST: TPM2_HC = TPM2_POLICY_SESSION_LAST; /* used in GetCapability */
pub const TPM2_TRANSIENT_LAST: TPM2_HC = TPM2_TRANSIENT_FIRST + 0x00fffffe; /* last transient object */
pub const TPM2_PERSISTENT_FIRST: TPM2_HC = TPM2_HR_PERSISTENT + 0; /* first persistent object */
pub const TPM2_PERSISTENT_LAST: TPM2_HC = TPM2_PERSISTENT_FIRST + 0x00FFFFFF; /* last persistent object */
pub const TPM2_PLATFORM_PERSISTENT: TPM2_HC = TPM2_PERSISTENT_FIRST + 0x00800000; /* first platform persistent object */
pub const TPM2_NV_INDEX_FIRST: TPM2_HC = TPM2_HR_NV_INDEX + 0; /* first allowed NV Index */
pub const TPM2_NV_INDEX_LAST: TPM2_HC = TPM2_NV_INDEX_FIRST + 0x00FFFFFF; /* last allowed NV Index */
pub const TPM2_PERMANENT_FIRST: TPM2_HC = TPM2_RH_FIRST;
pub const TPM2_PERMANENT_LAST: TPM2_HC = TPM2_RH_LAST;
pub const TPM2_HR_NV_AC: TPM2_HC = ((TPM2_HT_NV_INDEX as TPM2_HC) << TPM2_HR_SHIFT) + 0x00D00000; /* AC aliased NV Index */
pub const TPM2_NV_AC_FIRST: TPM2_HC = TPM2_HR_NV_AC + 0; /* first NV Index aliased to Attached Component */
pub const TPM2_NV_AC_LAST: TPM2_HC = TPM2_HR_NV_AC + 0x0000FFFF; /* last NV Index aliased to Attached Component */
pub const TPM2_HR_AC: TPM2_HC = (TPM2_HT_AC as TPM2_HC) << TPM2_HR_SHIFT; /* AC Handle */
pub const TPM2_AC_FIRST: TPM2_HC = TPM2_HR_AC + 0; /* first Attached Component */
pub const TPM2_AC_LAST: TPM2_HC = TPM2_HR_AC + 0x0000FFFF; /* last Attached Component */

pub const TPMA_ALGORITHM_ASYMMETRIC: TPMA_ALGORITHM = 0x00000001; /* SET 1 an asymmetric algorithm with public and private portions. CLEAR 0 not an asymmetric algorithm */
pub const TPMA_ALGORITHM_SYMMETRIC: TPMA_ALGORITHM = 0x00000002; /* SET 1 a symmetric block cipher. CLEAR 0 not a symmetric block cipher */
pub const TPMA_ALGORITHM_HASH: TPMA_ALGORITHM = 0x00000004; /* SET 1 a hash algorithm. CLEAR 0 not a hash algorithm */
pub const TPMA_ALGORITHM_OBJECT: TPMA_ALGORITHM = 0x00000008; /* SET 1 an algorithm that may be used as an object type. CLEAR 0 an algorithm that is not used as an object type */
pub const TPMA_ALGORITHM_RESERVED1_MASK: TPMA_ALGORITHM = 0x000000F0;
pub const TPMA_ALGORITHM_SIGNING: TPMA_ALGORITHM = 0x00000100; /* SET 1 a signing algorithm. The setting of asymmetric symmetric and hash will indicate the type of signing algorithm. CLEAR 0 not a signing algorithm */
pub const TPMA_ALGORITHM_ENCRYPTING: TPMA_ALGORITHM = 0x00000200; /* SET 1 an encryptiondecryption algorithm. The setting of asymmetric symmetric and hash will indicate the type of encryptiondecryption algorithm. CLEAR 0 not an encryption-decryption algorithm */
pub const TPMA_ALGORITHM_METHOD: TPMA_ALGORITHM = 0x00000400; /* SET 1 a method such as a key derivative function KDF. CLEAR 0 not a method */
pub const TPMA_ALGORITHM_RESERVED2_MASK: TPMA_ALGORITHM = 0xFFFFF800;

pub const TPMA_OBJECT_RESERVED1_MASK: TPMA_OBJECT = 0x00000001; /* shall be zero */
pub const TPMA_OBJECT_FIXEDTPM: TPMA_OBJECT = 0x00000002; /* SET 1 The hierarchy of the object as indicated by its Qualified Name may not change. CLEAR 0 The hierarchy of the object may change as a result of this object or an ancestor key being duplicated for use in another hierarchy. */
pub const TPMA_OBJECT_STCLEAR: TPMA_OBJECT = 0x00000004; /* SET 1 Previously saved contexts of this object may not be loaded after StartupCLEAR. CLEAR 0 Saved contexts of this object may be used after a ShutdownSTATE and subsequent Startup. */
pub const TPMA_OBJECT_RESERVED2_MASK: TPMA_OBJECT = 0x00000008; /* shall be zero */
pub const TPMA_OBJECT_FIXEDPARENT: TPMA_OBJECT = 0x00000010; /* SET 1 The parent of the object may not change. CLEAR 0 The parent of the object may change as the result of a TPM2_Duplicate of the object. */
pub const TPMA_OBJECT_SENSITIVEDATAORIGIN: TPMA_OBJECT = 0x00000020; /* SET 1 Indicates that when the object was created with TPM2_Create or TPM2_CreatePrimary the TPM generated all of the sensitive data other than the authValue. CLEAR 0 A portion of the sensitive data other than the authValue was provided by the caller. */
pub const TPMA_OBJECT_USERWITHAUTH: TPMA_OBJECT = 0x00000040; /* SET 1 Approval of USER role actions with this object may be with an HMAC session or with a password using the authValue of the object or a policy session. CLEAR 0 Approval of USER role actions with this object may only be done with a policy session. */
pub const TPMA_OBJECT_ADMINWITHPOLICY: TPMA_OBJECT = 0x00000080; /* SET 1 Approval of ADMIN role actions with this object may only be done with a policy session. CLEAR 0 Approval of ADMIN role actions with this object may be with an HMAC session or with a password using the authValue of the object or a policy session. */
pub const TPMA_OBJECT_RESERVED3_MASK: TPMA_OBJECT = 0x00000300; /* shall be zero */
pub const TPMA_OBJECT_NODA: TPMA_OBJECT = 0x00000400; /* SET 1 The object is not subject to dictionary attack protections. CLEAR 0 The object is subject to dictionary attack protections. */
pub const TPMA_OBJECT_ENCRYPTEDDUPLICATION: TPMA_OBJECT = 0x00000800; /* SET 1 If the object is duplicated then symmetricAlg shall not be TPM2_ALG_NULL and newParentHandle shall not be TPM2_RH_NULL. CLEAR 0 The object may be duplicated without an inner wrapper on the private portion of the object and the new parent may be TPM2_RH_NULL. */
pub const TPMA_OBJECT_RESERVED4_MASK: TPMA_OBJECT = 0x0000F000; /* shall be zero */
pub const TPMA_OBJECT_RESTRICTED: TPMA_OBJECT = 0x00010000; /* SET 1 Key usage is restricted to manipulate structures of known format the parent of this key shall have restricted SET. CLEAR 0 Key usage is not restricted to use on special formats. */
pub const TPMA_OBJECT_DECRYPT: TPMA_OBJECT = 0x00020000; /* SET 1 The private portion of the key may be used to decrypt. CLEAR 0 The private portion of the key may not be used to decrypt. */
pub const TPMA_OBJECT_SIGN_ENCRYPT: TPMA_OBJECT = 0x00040000; /* SET 1 For a symmetric cipher object the private portion of the key may be used to encrypt.  For other objects the private portion of the key may be used to sign. CLEAR 0 The private portion of the key may not be used to sign or encrypt. */
pub const TPMA_OBJECT_RESERVED5_MASK: TPMA_OBJECT = 0xFFF80000; /* shall be zero */

pub const TPMA_SESSION_CONTINUESESSION: TPMA_SESSION = 0x00000001; /* SET 1 In a command this setting indicates that the session is to remain active after successful completion of the command. In a response it indicates that the session is still active. If SET in the command this attribute shall be SET in the response. CLEAR 0 In a command this setting indicates that the TPM should close the session and flush any related context when the command completes successfully. In a response it indicates that the session is closed and the context is no longer active. This attribute has no meaning for a password authorization and the TPM will allow any setting of the attribute in the command and SET the attribute in the response. This attribute will only be CLEAR in one response for a logical session. If the attribute is CLEAR the context associated with the session is no longer in use and the space is available. A session created after another session is ended may have the same handle but logically is not the same session. This attribute has no effect if the command does not complete successfully. */
pub const TPMA_SESSION_AUDITEXCLUSIVE: TPMA_SESSION = 0x00000002; /* SET 1 In a command this setting indicates that the command should only be executed if the session is exclusive at the start of the command. In a response it indicates that the session is exclusive. This setting is only allowed if the audit attribute is SET TPM2_RC_ATTRIBUTES. CLEAR 0 In a command indicates that the session need not be exclusive at the start of the command.  In a response indicates that the session is not exclusive. In this revision if audit is CLEAR auditExclusive must be CLEAR in the command and will be CLEAR in the response.  In a future revision this bit may have a different meaning if audit is CLEAR. See Exclusive Audit Session clause in TPM 2.0 Part 1. */
pub const TPMA_SESSION_AUDITRESET: TPMA_SESSION = 0x00000004; /* SET 1 In a command this setting indicates that the audit digest of the session should be initialized and the exclusive status of the session SET. This setting is only allowed if the audit attribute is SET TPM2_RC_ATTRIBUTES. CLEAR 0 In a command indicates that the audit digest should not be initialized. This bit is always CLEAR in a response. In this revision if audit is CLEAR auditReset must be clear in the command and will be CLEAR in the response.  In a future revision this bit may have a different meaning if audit is CLEAR. */
pub const TPMA_SESSION_RESERVED1_MASK: TPMA_SESSION = 0x00000018; /* shall be CLEAR */
pub const TPMA_SESSION_DECRYPT: TPMA_SESSION = 0x00000020; /* SET 1 In a command this setting indicates that the first parameter in the command is symmetrically encrypted using the parameter encryption scheme described in TPM 2.0 Part 1. The TPM will decrypt the parameter after performing any HMAC computations and before unmarshaling the parameter. In a response the attribute is copied from the request but has no effect on the response. CLEAR 0 Session not used for encryption. For a password authorization this attribute will be CLEAR in both the command and response. This attribute may only be SET in one session per command. This attribute may be SET in a session that is not associated with a command handle. Such a session is provided for purposes of encrypting a parameter and not for authorization. This attribute may be SET in combination with any other session attributes. This attribute may only be SET if the first parameter of the command is a sized buffer TPM2B_. */
pub const TPMA_SESSION_ENCRYPT: TPMA_SESSION = 0x00000040; /* SET 1 In a command this setting indicates that the TPM should use this session to encrypt the first parameter in the response. In a response it indicates that the attribute was set in the command and that the TPM used the session to encrypt the first parameter in the response using the parameter encryption scheme described in TPM 2.0 Part 1. CLEAR 0 Session not used for encryption. For a password authorization this attribute will be CLEAR in both the command and response. This attribute may only be SET in one session per command. This attribute may be SET in a session that is not associated with a command handle. Such a session is provided for purposes of encrypting a parameter and not for authorization. This attribute may only be SET if the first parameter of a response is a sized buffer TPM2B_. */
pub const TPMA_SESSION_AUDIT: TPMA_SESSION = 0x00000080; /* SET 1 In a command or response this setting indicates that the session is for audit and that auditExclusive and auditReset have meaning. This session may also be used for authorization encryption or decryption. The encrypted and encrypt fields may be SET or CLEAR. CLEAR 0 Session is not used for audit. This attribute may only be SET in one session per command or response. If SET in the command then this attribute will be SET in the response. */

pub const TPMA_LOCALITY_TPM2_LOC_ZERO: TPMA_LOCALITY = 0x00000001;
pub const TPMA_LOCALITY_TPM2_LOC_ONE: TPMA_LOCALITY = 0x00000002;
pub const TPMA_LOCALITY_TPM2_LOC_TWO: TPMA_LOCALITY = 0x00000004;
pub const TPMA_LOCALITY_TPM2_LOC_THREE: TPMA_LOCALITY = 0x00000008;
pub const TPMA_LOCALITY_TPM2_LOC_FOUR: TPMA_LOCALITY = 0x00000010;
pub const TPMA_LOCALITY_EXTENDED_MASK: TPMA_LOCALITY = 0x000000E0; /* If any of these bits is set an extended locality is indicated */
pub const TPMA_LOCALITY_EXTENDED_SHIFT: u32 = 5;

pub const TPMA_PERMANENT_OWNERAUTHSET: TPMA_PERMANENT = 0x00000001; /* SET 1 TPM2_HierarchyChangeAuth with ownerAuth has been executed since the last TPM2_Clear. CLEAR 0 ownerAuth has not been changed since TPM2_Clear. */
pub const TPMA_PERMANENT_ENDORSEMENTAUTHSET: TPMA_PERMANENT = 0x00000002; /* SET 1 TPM2_HierarchyChangeAuth with endorsementAuth has been executed since the last TPM2_Clear. CLEAR 0 endorsementAuth has not been changed since TPM2_Clear. */
pub const TPMA_PERMANENT_LOCKOUTAUTHSET: TPMA_PERMANENT = 0x00000004; /* SET 1 TPM2_HierarchyChangeAuth with lockoutAuth has been executed since the last TPM2_Clear. CLEAR 0 lockoutAuth has not been changed since TPM2_Clear. */
pub const TPMA_PERMANENT_RESERVED1_MASK: TPMA_PERMANENT = 0x000000F8;
pub const TPMA_PERMANENT_DISABLECLEAR: TPMA_PERMANENT = 0x00000100; /* SET 1 TPM2_Clear is disabled. CLEAR 0 TPM2_Clear is enabled. NOTE See TPM2_ClearControl in TPM 2.0 Part 3 for details on changing this attribute. */
pub const TPMA_PERMANENT_INLOCKOUT: TPMA_PERMANENT = 0x00000200; /* SET 1 The TPM is in lockout and commands that require authorization with other than Platform Authorization or Lockout Authorization will not succeed. */
pub const TPMA_PERMANENT_TPMGENERATEDEPS: TPMA_PERMANENT = 0x00000400; /* SET 1 The EPS was created by the TPM. CLEAR 0 The EPS was created outside of the TPM using a manufacturer specific process. */
pub const TPMA_PERMANENT_RESERVED2_MASK: TPMA_PERMANENT = 0xFFFFF800;

pub const TPMA_STARTUP_CLEAR_PHENABLE: TPMA_STARTUP_CLEAR = 0x00000001; /* SET 1 The platform hierarchy is enabled and platformAuth or platformPolicy may be used for authorization. CLEAR 0 platformAuth and platformPolicy may not be used for authorizations and objects in the platform hierarchy including persistent objects cannot be used. NOTE See TPM2_HierarchyControl in TPM 2.0 Part 3 for details on changing this attribute. */
pub const TPMA_STARTUP_CLEAR_SHENABLE: TPMA_STARTUP_CLEAR = 0x00000002; /* SET 1 The Storage hierarchy is enabled and ownerAuth or ownerPolicy may be used for authorization. NV indices defined using owner authorization are accessible. CLEAR 0 ownerAuth and ownerPolicy may not be used for authorizations and objects in the Storage hierarchy persistent objects and NV indices defined using owner authorization cannot be used. NOTE See TPM2_HierarchyControl in TPM 2.0 Part 3 for details on changing this attribute. */
pub const TPMA_STARTUP_CLEAR_EHENABLE: TPMA_STARTUP_CLEAR = 0x00000004; /* SET 1 The EPS hierarchy is enabled and Endorsement Authorization may be used to authorize commands. CLEAR 0 Endorsement Authorization may not be used for authorizations and objects in the endorsement hierarchy including persistent objects cannot be used. NOTE See TPM2_HierarchyControl in TPM 2.0 Part 3 for details on changing this attribute. */
pub const TPMA_STARTUP_CLEAR_PHENABLENV: TPMA_STARTUP_CLEAR = 0x00000008; /* SET 1 NV indices that have TPMA_PLATFORM_CREATE SET may be read or written. The platform can create define and undefine indices. CLEAR 0 NV indices that have TPMA_PLATFORM_CREATE SET may not be read or written TPM2_RC_HANDLE. The platform cannot  define TPM2_RC_HIERARCHY or undefined TPM2_RC_HANDLE indices. NOTE See TPM2_HierarchyControl in TPM 2.0 Part 3 for details on changing this attribute. NOTE read refers to these commands TPM2_NV_Read TPM2_NV_ReadPublic TPM_NV_Certify TPM2_PolicyNVwrite refers to these commands TPM2_NV_Write TPM2_NV_Increment TPM2_NV_Extend TPM2_NV_SetBitsNOTE The TPM must query the index TPMA_PLATFORM_CREATE attribute to determine whether phEnableNV is applicable. Since the TPM will return TPM2_RC_HANDLE if the index does not exist it also returns this error code if the index is disabled. Otherwise the TPM would leak the existence of an index even when disabled. */
pub const TPMA_STARTUP_CLEAR_RESERVED1_MASK: TPMA_STARTUP_CLEAR = 0x7FFFFFF0; /* shall be zero */
pub const TPMA_STARTUP_CLEAR_ORDERLY: TPMA_STARTUP_CLEAR = 0x80000000; /* SET 1 The TPM received a TPM2_Shutdown and a matching TPM2_Startup. CLEAR 0 TPM2_StartupTPM2_SU_CLEAR was not preceded by a TPM2_Shutdown of any type. NOTE A shutdown is orderly if the TPM receives a TPM2_Shutdown of any type followed by a TPM2_Startup of any type. However the TPM will return an error if TPM2_StartupTPM2_SU_STATE was not preceded by TPM2_State_SaveTPM2_SU_STATE. */

pub const TPMA_MEMORY_SHAREDRAM: TPMA_MEMORY = 0x00000001; /* SET 1 indicates that the RAM memory used for authorization session contexts is shared with the memory used for transient objects. CLEAR 0 indicates that the memory used for authorization sessions is not shared with memory used for transient objects */
pub const TPMA_MEMORY_SHAREDNV: TPMA_MEMORY = 0x00000002; /* SET 1 indicates that the NV memory used for persistent objects is shared with the NV memory used for NV Index values. CLEAR 0 indicates that the persistent objects and NV Index values are allocated from separate sections of NV */
pub const TPMA_MEMORY_OBJECTCOPIEDTORAM: TPMA_MEMORY = 0x00000004; /* SET 1 indicates that the TPM copies persistent objects to a transientobject slot in RAM when the persistent object is referenced in a command. The TRM is required to make sure that an object slot is available. CLEAR 0 indicates that the TPM does not use transientobject slots when persistent objects are referenced */
pub const TPMA_MEMORY_RESERVED1_MASK: TPMA_MEMORY = 0xFFFFFFF8; /* shall be zero */

pub const TPMA_CC_COMMANDINDEX_MASK: TPMA_CC = 0x0000FFFF; /* indicates the command being selected */
pub const TPMA_CC_COMMANDINDEX_SHIFT: u32 = 0;
pub const TPMA_CC_RESERVED1_MASK: TPMA_CC = 0x003F0000; /* shall be zero */
pub const TPMA_CC_NV: TPMA_CC = 0x00400000; /* SET 1 indicates that the command may write to NV. CLEAR 0 indicates that the command does not write to NV */
pub const TPMA_CC_EXTENSIVE: TPMA_CC = 0x00800000; /* SET 1 This command could flush any number of loaded contexts. CLEAR 0 no additional changes other than indicated by the flushed attribute */
pub const TPMA_CC_FLUSHED: TPMA_CC = 0x01000000; /* SET 1 The context associated with any transient handle in the command will be flushed when this command completes. CLEAR 0 No context is flushed as a side effect of this command. */
pub const TPMA_CC_CHANDLES_MASK: TPMA_CC = 0x0E000000; /* indicates the number of the handles in the handle area for this command */
pub const TPMA_CC_CHANDLES_SHIFT: u32 = 25;
pub const TPMA_CC_RHANDLE: TPMA_CC = 0x10000000; /* SET 1 indicates the presence of the handle area in the response */
pub const TPMA_CC_V: TPMA_CC = 0x20000000; /* SET 1 indicates that the command is vendor-specific. CLEAR 0 indicates that the command is defined in a version of this specification */
pub const TPMA_CC_RES_MASK: TPMA_CC = 0xC0000000; /* allocated for software shall be zero */
pub const TPMA_CC_RES_SHIFT: u32 = 30;

pub const TPMI_DH_SAVED_TRANSIENT: TPMI_DH_SAVED = 0x80000000; /* an ordinary transient object */
pub const TPMI_DH_SAVED_SEQUENCE: TPMI_DH_SAVED = 0x80000001; /* a sequence object */
pub const TPMI_DH_SAVED_TRANSIENT_CLEAR: TPMI_DH_SAVED = 0x80000002; /* a transient object with the stClear attribute SET */

pub const TSS2_RC_LAYER_MASK: u32 = 0x00FF0000;

pub const TSS2_TPM_RC_LAYER: TSS2_RC = 0x00000000; /* base is a TPM2_RC_* */
pub const TSS2_FEATURE_RC_LAYER: TSS2_RC = 0x00060000; /* base is a TSS2_BASE_RC_* */
pub const TSS2_ESYS_RC_LAYER: TSS2_RC = 0x00070000; /* base is a TSS2_BASE_RC_* */
pub const TSS2_SYS_RC_LAYER: TSS2_RC = 0x00080000; /* base is a TSS2_BASE_RC_* */
pub const TSS2_MU_RC_LAYER: TSS2_RC = 0x00090000; /* base is a TSS2_BASE_RC_* */
pub const TSS2_TCTI_RC_LAYER: TSS2_RC = 0x000A0000; /* base is a TSS2_BASE_RC_* */
pub const TSS2_RESMGR_RC_LAYER: TSS2_RC = 0x000B0000; /* base is a TSS2_BASE_RC_* */
pub const TSS2_RESMGR_TPM_RC_LAYER: TSS2_RC = 0x000C0000; /* base is a TPM2_RC_* */

pub const TSS2_RC_SUCCESS: TSS2_RC = 0x00000000;

pub use crate::tss2_esys::TSS2_BASE_RC_ABI_MISMATCH; /* Passed in ABI version doesn't match called module's ABI version */
pub use crate::tss2_esys::TSS2_BASE_RC_BAD_CONTEXT; /* A context structure is bad */
pub use crate::tss2_esys::TSS2_BASE_RC_BAD_REFERENCE; /* A pointer is NULL that isn't allowed to be NULL. */
pub use crate::tss2_esys::TSS2_BASE_RC_BAD_SEQUENCE; /* Function called in the wrong order */
pub use crate::tss2_esys::TSS2_BASE_RC_BAD_SIZE; /* If size of a parameter is incorrect */
pub use crate::tss2_esys::TSS2_BASE_RC_BAD_TCTI_STRUCTURE; /* TCTI context is bad. */
pub use crate::tss2_esys::TSS2_BASE_RC_BAD_TR; /* Invalid ESYS_TR handle */
pub use crate::tss2_esys::TSS2_BASE_RC_BAD_VALUE; /* A parameter has a bad value */
pub use crate::tss2_esys::TSS2_BASE_RC_GENERAL_FAILURE; /* Catch all for all errors not otherwise specified */
pub use crate::tss2_esys::TSS2_BASE_RC_INCOMPATIBLE_TCTI; /* Unknown or unusable TCTI version */
pub use crate::tss2_esys::TSS2_BASE_RC_INSUFFICIENT_BUFFER; /* A buffer isn't large enough */
pub use crate::tss2_esys::TSS2_BASE_RC_INSUFFICIENT_CONTEXT; /* Context not large enough */
pub use crate::tss2_esys::TSS2_BASE_RC_INSUFFICIENT_RESPONSE; /* Response is not long enough */
pub use crate::tss2_esys::TSS2_BASE_RC_INVALID_SESSIONS; /* The TPM command doesn't use the number of sessions provided by the caller */
pub use crate::tss2_esys::TSS2_BASE_RC_IO_ERROR; /* IO failure */
pub use crate::tss2_esys::TSS2_BASE_RC_MALFORMED_RESPONSE; /* Response is malformed */
pub use crate::tss2_esys::TSS2_BASE_RC_MEMORY; /* Memory allocation failed */
pub use crate::tss2_esys::TSS2_BASE_RC_MULTIPLE_DECRYPT_SESSIONS; /* More than one session with TPMA_SESSION_DECRYPT bit set */
pub use crate::tss2_esys::TSS2_BASE_RC_MULTIPLE_ENCRYPT_SESSIONS; /* More than one session with TPMA_SESSION_ENCRYPT bit set */
pub use crate::tss2_esys::TSS2_BASE_RC_NOT_IMPLEMENTED; /* If called functionality isn't implemented */
pub use crate::tss2_esys::TSS2_BASE_RC_NOT_PERMITTED; /* Operation not permitted. */
pub use crate::tss2_esys::TSS2_BASE_RC_NOT_SUPPORTED; /* Functionality not supported. */
pub use crate::tss2_esys::TSS2_BASE_RC_NO_CONNECTION; /* Fails to connect to next lower layer */
pub use crate::tss2_esys::TSS2_BASE_RC_NO_DECRYPT_PARAM; /* A session with its TPMA_SESSION_DECRYPT bit set was passed to a TPM command that doesn't support encryption of the first command parameter. */
pub use crate::tss2_esys::TSS2_BASE_RC_NO_ENCRYPT_PARAM; /* A session with its TPMA_SESSION_ENCRYPT bit set was passed to a TPM command that doesn't support encryption of the first response parameter. */
pub use crate::tss2_esys::TSS2_BASE_RC_RSP_AUTH_FAILED; /* Authorizing the TPM response failed */
pub use crate::tss2_esys::TSS2_BASE_RC_TRY_AGAIN; /* Operation timed out; function must be called again to be completed */

cfg_if::cfg_if! {
    if #[cfg(has_tss_base_rc_values_28_to_51_req)] {
        pub use crate::tss2_esys::TSS2_BASE_RC_NO_CONFIG; /* No config is available */
        pub use crate::tss2_esys::TSS2_BASE_RC_BAD_PATH; /* The provided path is bad */
        pub use crate::tss2_esys::TSS2_BASE_RC_NOT_DELETABLE; /* The object is not deletable */
        pub use crate::tss2_esys::TSS2_BASE_RC_PATH_ALREADY_EXISTS; /* The provided path already exists */
        pub use crate::tss2_esys::TSS2_BASE_RC_KEY_NOT_FOUND; /* The key was not found */
        pub use crate::tss2_esys::TSS2_BASE_RC_SIGNATURE_VERIFICATION_FAILED; /* Signature verification failed */
        pub use crate::tss2_esys::TSS2_BASE_RC_HASH_MISMATCH; /* Hashes mismatch */
        pub use crate::tss2_esys::TSS2_BASE_RC_KEY_NOT_DUPLICABLE; /* Key is not duplicatable */
        pub use crate::tss2_esys::TSS2_BASE_RC_PATH_NOT_FOUND; /* The path was not found */
        pub use crate::tss2_esys::TSS2_BASE_RC_NO_CERT; /* No certificate */
        pub use crate::tss2_esys::TSS2_BASE_RC_NO_PCR; /* No PCR */
        pub use crate::tss2_esys::TSS2_BASE_RC_PCR_NOT_RESETTABLE; /* PCR not resettable */
        pub use crate::tss2_esys::TSS2_BASE_RC_BAD_TEMPLATE; /* The template is bad */
        pub use crate::tss2_esys::TSS2_BASE_RC_AUTHORIZATION_FAILED; /* Authorization failed */
        pub use crate::tss2_esys::TSS2_BASE_RC_AUTHORIZATION_UNKNOWN; /* Authorization is unknown */
        pub use crate::tss2_esys::TSS2_BASE_RC_NV_NOT_READABLE; /* NV is not readable */
        pub use crate::tss2_esys::TSS2_BASE_RC_NV_TOO_SMALL; /* NV is too small */
        pub use crate::tss2_esys::TSS2_BASE_RC_NV_NOT_WRITEABLE; /* NV is not writable */
        pub use crate::tss2_esys::TSS2_BASE_RC_POLICY_UNKNOWN; /* The policy is unknown */
        pub use crate::tss2_esys::TSS2_BASE_RC_NV_WRONG_TYPE; /* The NV type is wrong */
        pub use crate::tss2_esys::TSS2_BASE_RC_NAME_ALREADY_EXISTS; /* The name already exists */
        pub use crate::tss2_esys::TSS2_BASE_RC_NO_TPM; /* No TPM available */
        pub use crate::tss2_esys::TSS2_BASE_RC_BAD_KEY; /* The key is bad */
        pub use crate::tss2_esys::TSS2_BASE_RC_NO_HANDLE; /* No handle provided */
    } else {
        pub const TSS2_BASE_RC_NO_CONFIG: u32 = 28; /* No config is available */
        pub const TSS2_BASE_RC_BAD_PATH: u32 = 29; /* The provided path is bad */
        pub const TSS2_BASE_RC_NOT_DELETABLE: u32 = 30; /* The object is not deletable */
        pub const TSS2_BASE_RC_PATH_ALREADY_EXISTS: u32 = 31; /* The provided path already exists */
        pub const TSS2_BASE_RC_KEY_NOT_FOUND: u32 = 32; /* The key was not found */
        pub const TSS2_BASE_RC_SIGNATURE_VERIFICATION_FAILED: u32 = 33; /* Signature verification failed */
        pub const TSS2_BASE_RC_HASH_MISMATCH: u32 = 34; /* Hashes mismatch */
        pub const TSS2_BASE_RC_KEY_NOT_DUPLICABLE: u32 = 35; /* Key is not duplicatable */
        pub const TSS2_BASE_RC_PATH_NOT_FOUND: u32 = 36; /* The path was not found */
        pub const TSS2_BASE_RC_NO_CERT: u32 = 37; /* No certificate */
        pub const TSS2_BASE_RC_NO_PCR: u32 = 38; /* No PCR */
        pub const TSS2_BASE_RC_PCR_NOT_RESETTABLE: u32 = 39; /* PCR not resettable */
        pub const TSS2_BASE_RC_BAD_TEMPLATE: u32 = 40; /* The template is bad */
        pub const TSS2_BASE_RC_AUTHORIZATION_FAILED: u32 = 41; /* Authorization failed */
        pub const TSS2_BASE_RC_AUTHORIZATION_UNKNOWN: u32 = 42; /* Authorization is unknown */
        pub const TSS2_BASE_RC_NV_NOT_READABLE: u32 = 43; /* NV is not readable */
        pub const TSS2_BASE_RC_NV_TOO_SMALL: u32 = 44; /* NV is too small */
        pub const TSS2_BASE_RC_NV_NOT_WRITEABLE: u32 = 45; /* NV is not writable */
        pub const TSS2_BASE_RC_POLICY_UNKNOWN: u32 = 46; /* The policy is unknown */
        pub const TSS2_BASE_RC_NV_WRONG_TYPE: u32 = 47; /* The NV type is wrong */
        pub const TSS2_BASE_RC_NAME_ALREADY_EXISTS: u32 = 48; /* The name already exists */
        pub const TSS2_BASE_RC_NO_TPM: u32 = 49; /* No TPM available */
        pub const TSS2_BASE_RC_BAD_KEY: u32 = 50; /* The key is bad */
        pub const TSS2_BASE_RC_NO_HANDLE: u32 = 51; /* No handle provided */
    }
}

cfg_if::cfg_if! {
    if #[cfg(has_tss_base_rc_values_52_to_53_req)] {
        pub use crate::tss2_esys::TSS2_BASE_RC_NOT_PROVISIONED; /* Provisioning was not executed */
        pub use crate::tss2_esys::TSS2_BASE_RC_ALREADY_PROVISIONED; /* Already provisioned */
    } else {
        pub const TSS2_BASE_RC_NOT_PROVISIONED: u32 = 52; /* Provisioning was not executed */
        pub const TSS2_BASE_RC_ALREADY_PROVISIONED: u32 = 53; /* Already provisioned */
    }
}
