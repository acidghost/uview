#ifndef _SYSINFO_H_
#define _SYSINFO_H_


typedef struct {
    unsigned long long total;
    unsigned long long free;

    unsigned long long buffers;
    unsigned long long cached;

    unsigned long long swap_total;
    unsigned long long swap_free;
} mem_info_t;



mem_info_t get_mem_info(void);


#endif /* _SYSINFO_H_ */
