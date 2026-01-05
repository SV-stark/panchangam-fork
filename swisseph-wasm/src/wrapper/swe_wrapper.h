// Standalone wrapper for bindgen - no external includes
// This ensures bindgen has no reason to fail parsing

// Basic types for bindgen mapping
typedef int int32_t;

#ifdef __cplusplus
extern "C" {
#endif

// Core functions we need - using standard types
// Core Calculation
int32_t swe_calc_ut(double tjd_ut, int32_t ipl, int32_t iflag, double *xx, char *serr);
int32_t swe_calc(double tjd, int32_t ipl, int32_t iflag, double *xx, char *serr);

// Fixed Stars
int32_t swe_fixstar_ut(char *star, double tjd_ut, int32_t iflag, double *xx, char *serr);
int32_t swe_fixstar(char *star, double tjd, int32_t iflag, double *xx, char *serr);

// Date & Time
double swe_julday(int year, int month, int day, double hour, int gregflag);
void swe_revjul(double tjd, int gregflag, int *year, int *month, int *day, double *hour);
int32_t swe_date_conversion(int year, int month, int day, double hour, char c, double *tjd);
double swe_sidtime(double tjd_ut);

// Phenomena & Nodes
int32_t swe_pheno_ut(double tjd_ut, int32_t ipl, int32_t iflag, double *attr, char *serr);
int32_t swe_nod_aps_ut(double tjd_ut, int32_t ipl, int32_t iflag, int32_t method, double *xn, double *xa, double *xp, char *serr);
void swe_azalt(double tjd_ut, int32_t calc_flag, double *geopos, double atpress, double attemp, double *xin, double *xaz);
int32_t swe_rise_trans(double tjd_ut, int32_t ipl, char *starname, int32_t epheflag, int32_t rsmi, double *geopos, double atpress, double attemp, double *tret, char *serr);

// Configuration
void swe_set_topo(double geolon, double geolat, double geoalt);
void swe_set_sid_mode(int32_t sid_mode, double t0, double ayan_t0);
void swe_set_ephe_path(char *path);
void swe_close(void);

// Auxiliary
double swe_deltat(double tjd);
char* swe_get_planet_name(int32_t ipl, char *spname);
double swe_get_ayanamsa_ut(double tjd_ut);

char* swe_version(char* s);

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
