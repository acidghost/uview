#include <sys/sysinfo.h>

#include "sysinfo.h"


mem_info_t get_mem_info(void)
{
    mem_info_t mi;
    memset(&mi, 0, sizeof(mem_info_t));

    sysinfo_t si;
    if (sysinfo(&si) == 0) {
        mi.total       = si.totalram * si.mem_unit;
        mi.free        = si.freeram * si.mem_unit;
        mi.buffers     = si.bufferram * si.mem_unit;
        mi.cached      = 0;
        mi.swap_total  = si.totalswap * si.mem_unit;
        mi.swap_free   = si.freeswap * si.mem_unit;
    }

    return mi;
}
