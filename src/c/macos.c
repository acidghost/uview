#include <stdlib.h>
#include <sys/sysctl.h>
#include <mach/mach_init.h>
#include <mach/mach_host.h>

#include "sysinfo.h"


mem_info_t get_mem_info(void)
{
    static unsigned long long size = 0;
    size_t len;
    int mib[2];
    vm_statistics_data_t vm_stat;
    mach_msg_type_number_t count = HOST_VM_INFO_COUNT;
    mem_info_t mi;

    if (size == 0) {
        mib[0] = CTL_HW;
        mib[1] = HW_MEMSIZE;
        len = sizeof(size);
        sysctl(mib, 2, &size, &len, NULL, 0);
    }

    host_statistics(mach_host_self(), HOST_VM_INFO,
                    (host_info_t) &vm_stat, &count);

    mi.total       = size;
    mi.free        = vm_stat.free_count * PAGE_SIZE;
    mi.buffers     = 0;
    mi.cached      = 0;
    mi.swap_total  = 0;
    mi.swap_free   = 0;

    return mi;
}
