#ifndef OMP_STUB_H
#define OMP_STUB_H
static inline int omp_get_max_threads(void){return 1;}
static inline int omp_get_num_threads(void){return 1;}
static inline int omp_get_thread_num(void){return 0;}
static inline void omp_set_num_threads(int n){(void)n;}
#endif
