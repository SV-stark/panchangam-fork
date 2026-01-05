// Standalone wrapper for bindgen - no external includes
// This ensures bindgen has no reason to fail parsing

// Basic types for bindgen mapping
typedef int int32_t;

#ifdef __cplusplus
extern "C" {
#endif

// Core functions we need - using standard types
char* swe_version(char *s);
double swe_julday(int year, int month, int day, double hour, int gregflag);
int32_t swe_calc_ut(double tjd_ut, int32_t ipl, int32_t iflag, double *xx, char *serr);
int32_t swe_pheno_ut(double tjd_ut, int32_t ipl, int32_t iflag, double *attr, char *serr);
void swe_set_sid_mode(int32_t sid_mode, double t0, double ayan_t0);
double swe_get_ayanamsa_ut(double tjd_ut);

#ifdef __cplusplus
}
#endif

// Constants - planet IDs
#define SE_SUN 0
#define SE_MOON 1
#define SE_GREG_CAL 1

// Constants - flags
#define SEFLG_SWIEPH 2
#define SEFLG_SPEED 256

// Constants - sidereal modes
#define SE_SIDM_LAHIRI 1
#define SE_SIDM_RAMAN 3
#define SE_SIDM_KRISHNAMURTI 5
#define SE_SIDM_TRUE_CITRA 27
