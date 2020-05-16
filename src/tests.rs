#![allow(dead_code)]
use super::*;

#[cfg(test)]
mod gets {
    use self::ps::*;
    use super::*;
    #[test]
    fn cpu_info() {
        assert_eq!(
            ProcFs::_cpu_info(&tests::CPU_INFO),
            "AMD Ryzen 5 3600 6-Core Processor".to_string()
        )
    }
    #[test]
    fn total_clock_speed() {
        assert_eq!(ProcFs::_total_clock_speed(&tests::CPU_INFO), 26040.395)
    }
    #[test]
    fn total_cpu_cores() {
        assert_eq!(ProcFs::_total_cpu_cores(&tests::CPU_INFO), 12)
    }
    #[test]
    fn mem_free() {
        assert_eq!(ProcFs::_mem(Memory::MemFree, &tests::MEM_INFO), 10178555904);
    }
    #[test]
    fn mem_total() {
        assert_eq!(
            ProcFs::_mem(Memory::MemTotal, &tests::MEM_INFO),
            16714952704
        );
    }
    #[test]
    fn swap_free() {
        assert_eq!(ProcFs::_mem(Memory::SwapFree, &tests::MEM_INFO), 0);
    }
    #[test]
    fn swap_total() {
        assert_eq!(ProcFs::_mem(Memory::SwapTotal, &tests::MEM_INFO), 0);
    }
    #[test]
    fn ipv4_addr() {
        assert_eq!(
            ProcFs::_ipv4_addr("wlan0", &tests::ROUTE, &tests::FIB_TRIE).unwrap(),
            Ipv4Addr::new(192, 168, 8, 201)
        );
        assert_eq!(
            ProcFs::_ipv4_addr("lo", &tests::ROUTE, &tests::FIB_TRIE).unwrap(),
            Ipv4Addr::new(127, 0, 0, 1)
        )
    }
    #[test]
    fn ipv6_addr() {
        assert_eq!(
            ProcFs::_ipv6_addr("wlan0", &tests::IF_INET6).unwrap(),
            Ipv6Addr::new(0xfe80, 0, 0, 0, 0xd81, 0x2a0d, 0x8467, 0xda1c)
        );
    }
    #[test]
    fn uptime() {
        assert_eq!(ProcFs::_uptime(&tests::UPTIME), 52662.34)
    }
    #[test]
    fn graphics_card() {
        assert_eq!(
            ProcFs::_graphics_card(&tests::LSPCI),
            "NVIDIA Corporation GK106 [GeForce GTX 660] (rev a1)"
        )
    }
    #[test]
    fn storage_partitions() {
        let partitions = vec![
            Partition {
                name: "sdd1".to_string(),
                major: 8,
                minor: 49,
                size: 524288000,
                filesystem: "vfat".to_string(),
                mountpoint: "/boot".to_string(),
            },
            Partition {
                name: "sdd2".to_string(),
                major: 8,
                minor: 50,
                size: 119508769792,
                filesystem: "".to_string(),
                mountpoint: "".to_string(),
            },
        ];
        assert_eq!(
            ProcFs::_storage_partitions("sdd", &tests::STOR_DEV, &tests::STOR_MOUNTS),
            partitions
        )
    }
    #[test]
    fn storage_devices() {
        let sdc_partitions = vec![Partition {
            name: "sdc1".to_string(),
            major: 8,
            minor: 33,
            size: 320070786048,
            filesystem: "".to_string(),
            mountpoint: "".to_string(),
        }];
        let sdd_partitions = vec![
            Partition {
                name: "sdd1".to_string(),
                major: 8,
                minor: 49,
                size: 524288000,
                filesystem: "vfat".to_string(),
                mountpoint: "/boot".to_string(),
            },
            Partition {
                name: "sdd2".to_string(),
                major: 8,
                minor: 50,
                size: 119508769792,
                filesystem: "".to_string(),
                mountpoint: "".to_string(),
            },
        ];
        let sde_partitions = vec![
            Partition {
                name: "sde1".to_string(),
                major: 8,
                minor: 65,
                size: 209715200,
                filesystem: "".to_string(),
                mountpoint: "".to_string(),
            },
            Partition {
                name: "sde2".to_string(),
                major: 8,
                minor: 66,
                size: 99931389952,
                filesystem: "".to_string(),
                mountpoint: "".to_string(),
            },
            Partition {
                name: "sde3".to_string(),
                major: 8,
                minor: 67,
                size: 99932839936,
                filesystem: "".to_string(),
                mountpoint: "".to_string(),
            },
            Partition {
                name: "sde4".to_string(),
                major: 8,
                minor: 68,
                size: 299629670400,
                filesystem: "".to_string(),
                mountpoint: "".to_string(),
            },
        ];
        let devices = vec![
            Storage {
                major: 8,
                minor: 32,
                size: 320071851008,
                name: "sdc".to_string(),
                partitions: sdc_partitions,
            },
            Storage {
                major: 8,
                minor: 48,
                size: 120034123776,
                name: "sdd".to_string(),
                partitions: sdd_partitions,
            },
            Storage {
                major: 8,
                minor: 64,
                size: 500107862016,
                name: "sde".to_string(),
                partitions: sde_partitions,
            },
        ];
        let storages = Storages {
            storage_devices: devices,
        };
        assert_eq!(
            ProcFs::_storage_devices(&tests::STOR_DEV, &tests::STOR_MOUNTS),
            storages
        );
    }
    #[test]
    fn network_devices() {
        let net_dev = NetworkDevices {
            net_devices: vec![
                NetworkDevice {
                    name: "lo".to_string(),
                    received_bytes: 817348,
                    transfered_bytes: 817348,
                    ipv4_addr: Ipv4Addr::new(127, 0, 0, 1),
                    ipv6_addr: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
                },
                NetworkDevice {
                    name: "enp8s0".to_string(),
                    received_bytes: 0,
                    transfered_bytes: 0,
                    ipv4_addr: Ipv4Addr::new(0, 0, 0, 0),
                    ipv6_addr: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
                },
                NetworkDevice {
                    name: "wlan0".to_string(),
                    received_bytes: 1177144648,
                    transfered_bytes: 59578768,
                    ipv4_addr: Ipv4Addr::new(192, 168, 8, 201),
                    ipv6_addr: Ipv6Addr::new(0xfe80, 0, 0, 0, 0xd81, 0x2a0d, 0x8467, 0xda1c),
                },
                NetworkDevice {
                    name: "tun0".to_string(),
                    received_bytes: 24156600,
                    transfered_bytes: 3623219,
                    ipv4_addr: Ipv4Addr::new(0, 0, 0, 0),
                    ipv6_addr: Ipv6Addr::new(0xfe80, 0, 0, 0, 0xf17b, 0x7100, 0xb5a1, 0xf781),
                },
            ],
        };
        assert_eq!(
            ProcFs::_network_dev(
                &tests::NET_DEV,
                &tests::ROUTE,
                &tests::FIB_TRIE,
                &tests::IF_INET6
            )
            .unwrap(),
            net_dev
        )
    }
    #[test]
    fn process() {
        let base = Process {
            pid: 17008,
            name: "htop".to_string(),
            cmd: "htop -t --no-color".to_string(),
            state: ProcessState::Sleeping,
            ppid: 5868,
            pgrp: 17008,
            utime: 1169,
            stime: 4309,
            size: 2926,
            resident: 1266,
            shared: 827,
            uid: 0,
            gid: 0,
        };

        let mut proc = Process::default();
        proc.parse_proc_stat(&PROC_STAT).unwrap();
        proc.parse_proc_statm(&PROC_STATM).unwrap();
        proc.cmd = Process::_cmd(&PROC_CMDLINE);

        assert_eq!(base, proc);
    }
}

static CPU_INFO: &str = "processor       : 0
vendor_id       : AuthenticAMD
cpu family      : 23
model           : 113
model name      : AMD Ryzen 5 3600 6-Core Processor
stepping        : 0
microcode       : 0x8701013
cpu MHz         : 2541.363
cache size      : 512 KB
physical id     : 0
siblings        : 12
core id         : 0
cpu cores       : 6
apicid          : 0
initial apicid  : 0
fpu             : yes
fpu_exception   : yes
cpuid level     : 16
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate sme ssbd mba sev ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr rdpru wbnoinvd arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif umip rdpid overflow_recov succor smca
bugs            : sysret_ss_attrs spectre_v1 spectre_v2 spec_store_bypass
bogomips        : 7189.01
TLB size        : 3072 4K pages
clflush size    : 64
cache_alignment : 64
address sizes   : 43 bits physical, 48 bits virtual
power management: ts ttp tm hwpstate eff_freq_ro [13] [14]

processor       : 1
vendor_id       : AuthenticAMD
cpu family      : 23
model           : 113
model name      : AMD Ryzen 5 3600 6-Core Processor
stepping        : 0
microcode       : 0x8701013
cpu MHz         : 1953.038
cache size      : 512 KB
physical id     : 0
siblings        : 12
core id         : 1
cpu cores       : 6
apicid          : 2
initial apicid  : 2
fpu             : yes
fpu_exception   : yes
cpuid level     : 16
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate sme ssbd mba sev ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr rdpru wbnoinvd arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif umip rdpid overflow_recov succor smca
bugs            : sysret_ss_attrs spectre_v1 spectre_v2 spec_store_bypass
bogomips        : 7189.01
TLB size        : 3072 4K pages
clflush size    : 64
cache_alignment : 64
address sizes   : 43 bits physical, 48 bits virtual
power management: ts ttp tm hwpstate eff_freq_ro [13] [14]

processor       : 2
vendor_id       : AuthenticAMD
cpu family      : 23
model           : 113
model name      : AMD Ryzen 5 3600 6-Core Processor
stepping        : 0
microcode       : 0x8701013
cpu MHz         : 1888.751
cache size      : 512 KB
physical id     : 0
siblings        : 12
core id         : 2
cpu cores       : 6
apicid          : 4
initial apicid  : 4
fpu             : yes
fpu_exception   : yes
cpuid level     : 16
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate sme ssbd mba sev ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr rdpru wbnoinvd arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif umip rdpid overflow_recov succor smca
bugs            : sysret_ss_attrs spectre_v1 spectre_v2 spec_store_bypass
bogomips        : 7189.01
TLB size        : 3072 4K pages
clflush size    : 64
cache_alignment : 64
address sizes   : 43 bits physical, 48 bits virtual
power management: ts ttp tm hwpstate eff_freq_ro [13] [14]

processor       : 3
vendor_id       : AuthenticAMD
cpu family      : 23
model           : 113
model name      : AMD Ryzen 5 3600 6-Core Processor
stepping        : 0
microcode       : 0x8701013
cpu MHz         : 2336.654
cache size      : 512 KB
physical id     : 0
siblings        : 12
core id         : 4
cpu cores       : 6
apicid          : 8
initial apicid  : 8
fpu             : yes
fpu_exception   : yes
cpuid level     : 16
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate sme ssbd mba sev ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr rdpru wbnoinvd arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif umip rdpid overflow_recov succor smca
bugs            : sysret_ss_attrs spectre_v1 spectre_v2 spec_store_bypass
bogomips        : 7189.01
TLB size        : 3072 4K pages
clflush size    : 64
cache_alignment : 64
address sizes   : 43 bits physical, 48 bits virtual
power management: ts ttp tm hwpstate eff_freq_ro [13] [14]

processor       : 4
vendor_id       : AuthenticAMD
cpu family      : 23
model           : 113
model name      : AMD Ryzen 5 3600 6-Core Processor
stepping        : 0
microcode       : 0x8701013
cpu MHz         : 1886.176
cache size      : 512 KB
physical id     : 0
siblings        : 12
core id         : 5
cpu cores       : 6
apicid          : 10
initial apicid  : 10
fpu             : yes
fpu_exception   : yes
cpuid level     : 16
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate sme ssbd mba sev ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr rdpru wbnoinvd arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif umip rdpid overflow_recov succor smca
bugs            : sysret_ss_attrs spectre_v1 spectre_v2 spec_store_bypass
bogomips        : 7189.01
TLB size        : 3072 4K pages
clflush size    : 64
cache_alignment : 64
address sizes   : 43 bits physical, 48 bits virtual
power management: ts ttp tm hwpstate eff_freq_ro [13] [14]

processor       : 5
vendor_id       : AuthenticAMD
cpu family      : 23
model           : 113
model name      : AMD Ryzen 5 3600 6-Core Processor
stepping        : 0
microcode       : 0x8701013
cpu MHz         : 1883.618
cache size      : 512 KB
physical id     : 0
siblings        : 12
core id         : 6
cpu cores       : 6
apicid          : 12
initial apicid  : 12
fpu             : yes
fpu_exception   : yes
cpuid level     : 16
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate sme ssbd mba sev ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr rdpru wbnoinvd arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif umip rdpid overflow_recov succor smca
bugs            : sysret_ss_attrs spectre_v1 spectre_v2 spec_store_bypass
bogomips        : 7189.01
TLB size        : 3072 4K pages
clflush size    : 64
cache_alignment : 64
address sizes   : 43 bits physical, 48 bits virtual
power management: ts ttp tm hwpstate eff_freq_ro [13] [14]

processor       : 6
vendor_id       : AuthenticAMD
cpu family      : 23
model           : 113
model name      : AMD Ryzen 5 3600 6-Core Processor
stepping        : 0
microcode       : 0x8701013
cpu MHz         : 2779.924
cache size      : 512 KB
physical id     : 0
siblings        : 12
core id         : 0
cpu cores       : 6
apicid          : 1
initial apicid  : 1
fpu             : yes
fpu_exception   : yes
cpuid level     : 16
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate sme ssbd mba sev ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr rdpru wbnoinvd arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif umip rdpid overflow_recov succor smca
bugs            : sysret_ss_attrs spectre_v1 spectre_v2 spec_store_bypass
bogomips        : 7189.01
TLB size        : 3072 4K pages
clflush size    : 64
cache_alignment : 64
address sizes   : 43 bits physical, 48 bits virtual
power management: ts ttp tm hwpstate eff_freq_ro [13] [14]

processor       : 7
vendor_id       : AuthenticAMD
cpu family      : 23
model           : 113
model name      : AMD Ryzen 5 3600 6-Core Processor
stepping        : 0
microcode       : 0x8701013
cpu MHz         : 1977.488
cache size      : 512 KB
physical id     : 0
siblings        : 12
core id         : 1
cpu cores       : 6
apicid          : 3
initial apicid  : 3
fpu             : yes
fpu_exception   : yes
cpuid level     : 16
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate sme ssbd mba sev ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr rdpru wbnoinvd arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif umip rdpid overflow_recov succor smca
bugs            : sysret_ss_attrs spectre_v1 spectre_v2 spec_store_bypass
bogomips        : 7189.01
TLB size        : 3072 4K pages
clflush size    : 64
cache_alignment : 64
address sizes   : 43 bits physical, 48 bits virtual
power management: ts ttp tm hwpstate eff_freq_ro [13] [14]

processor       : 8
vendor_id       : AuthenticAMD
cpu family      : 23
model           : 113
model name      : AMD Ryzen 5 3600 6-Core Processor
stepping        : 0
microcode       : 0x8701013
cpu MHz         : 1874.803
cache size      : 512 KB
physical id     : 0
siblings        : 12
core id         : 2
cpu cores       : 6
apicid          : 5
initial apicid  : 5
fpu             : yes
fpu_exception   : yes
cpuid level     : 16
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate sme ssbd mba sev ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr rdpru wbnoinvd arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif umip rdpid overflow_recov succor smca
bugs            : sysret_ss_attrs spectre_v1 spectre_v2 spec_store_bypass
bogomips        : 7189.01
TLB size        : 3072 4K pages
clflush size    : 64
cache_alignment : 64
address sizes   : 43 bits physical, 48 bits virtual
power management: ts ttp tm hwpstate eff_freq_ro [13] [14]

processor       : 9
vendor_id       : AuthenticAMD
cpu family      : 23
model           : 113
model name      : AMD Ryzen 5 3600 6-Core Processor
stepping        : 0
microcode       : 0x8701013
cpu MHz         : 2958.528
cache size      : 512 KB
physical id     : 0
siblings        : 12
core id         : 4
cpu cores       : 6
apicid          : 9
initial apicid  : 9
fpu             : yes
fpu_exception   : yes
cpuid level     : 16
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate sme ssbd mba sev ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr rdpru wbnoinvd arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif umip rdpid overflow_recov succor smca
bugs            : sysret_ss_attrs spectre_v1 spectre_v2 spec_store_bypass
bogomips        : 7189.01
TLB size        : 3072 4K pages
clflush size    : 64
cache_alignment : 64
address sizes   : 43 bits physical, 48 bits virtual
power management: ts ttp tm hwpstate eff_freq_ro [13] [14]

processor       : 10
vendor_id       : AuthenticAMD
cpu family      : 23
model           : 113
model name      : AMD Ryzen 5 3600 6-Core Processor
stepping        : 0
microcode       : 0x8701013
cpu MHz         : 1990.465
cache size      : 512 KB
physical id     : 0
siblings        : 12
core id         : 5
cpu cores       : 6
apicid          : 11
initial apicid  : 11
fpu             : yes
fpu_exception   : yes
cpuid level     : 16
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate sme ssbd mba sev ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr rdpru wbnoinvd arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif umip rdpid overflow_recov succor smca
bugs            : sysret_ss_attrs spectre_v1 spectre_v2 spec_store_bypass
bogomips        : 7189.01
TLB size        : 3072 4K pages
clflush size    : 64
cache_alignment : 64
address sizes   : 43 bits physical, 48 bits virtual
power management: ts ttp tm hwpstate eff_freq_ro [13] [14]

processor       : 11
vendor_id       : AuthenticAMD
cpu family      : 23
model           : 113
model name      : AMD Ryzen 5 3600 6-Core Processor
stepping        : 0
microcode       : 0x8701013
cpu MHz         : 1969.588
cache size      : 512 KB
physical id     : 0
siblings        : 12
core id         : 6
cpu cores       : 6
apicid          : 13
initial apicid  : 13
fpu             : yes
fpu_exception   : yes
cpuid level     : 16
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate sme ssbd mba sev ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr rdpru wbnoinvd arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif umip rdpid overflow_recov succor smca
bugs            : sysret_ss_attrs spectre_v1 spectre_v2 spec_store_bypass
bogomips        : 7189.01
TLB size        : 3072 4K pages
clflush size    : 64
cache_alignment : 64
address sizes   : 43 bits physical, 48 bits virtual
power management: ts ttp tm hwpstate eff_freq_ro [13] [14]";

static MEM_INFO: &str = "MemTotal:       16323196 kB
MemFree:         9939996 kB
MemAvailable:   12560252 kB
Buffers:          157600 kB
Cached:          2719788 kB
SwapCached:            0 kB
Active:          4417828 kB
Inactive:        1513532 kB
Active(anon):    3038952 kB
Inactive(anon):   130196 kB
Active(file):    1378876 kB
Inactive(file):  1383336 kB
Unevictable:          16 kB
Mlocked:              16 kB
SwapTotal:             0 kB
SwapFree:              0 kB
Dirty:               264 kB
Writeback:             0 kB
AnonPages:       3054272 kB
Mapped:           468476 kB
Shmem:            131960 kB
KReclaimable:     194012 kB
Slab:             300140 kB
SReclaimable:     194012 kB
SUnreclaim:       106128 kB
KernelStack:       12576 kB
PageTables:        21148 kB
NFS_Unstable:          0 kB
Bounce:                0 kB
WritebackTmp:          0 kB
CommitLimit:     8161596 kB
Committed_AS:    5440524 kB
VmallocTotal:   34359738367 kB
VmallocUsed:       47472 kB
VmallocChunk:          0 kB
Percpu:            15104 kB
HardwareCorrupted:     0 kB
AnonHugePages:         0 kB
ShmemHugePages:        0 kB
ShmemPmdMapped:        0 kB
FileHugePages:         0 kB
FilePmdMapped:         0 kB
HugePages_Total:       0
HugePages_Free:        0
HugePages_Rsvd:        0
HugePages_Surp:        0
Hugepagesize:       2048 kB
Hugetlb:               0 kB
DirectMap4k:      325424 kB
DirectMap2M:     6944768 kB
DirectMap1G:    10485760 kB";

static ROUTE: &str = "Iface   Destination     Gateway         Flags   RefCnt  Use     Metric  Mask            MTU     Window  IRTT          
wlan0   00000000        0108A8C0        0003    0       0       304     00000000        0       0       0             
wlan0   0008A8C0        00000000        0001    0       0       304     00FFFFFF        0       0       0";

static FIB_TRIE: &str = "Main:
  +-- 0.0.0.0/0 3 0 5
     |-- 0.0.0.0
        /0 universe UNICAST
     +-- 127.0.0.0/8 2 0 2
        +-- 127.0.0.0/31 1 0 0
           |-- 127.0.0.0
              /32 link BROADCAST
              /8 host LOCAL
           |-- 127.0.0.1
              /32 host LOCAL
        |-- 127.255.255.255
           /32 link BROADCAST
     +-- 192.168.8.0/24 2 1 2
        |-- 192.168.8.0
           /32 link BROADCAST
           /24 link UNICAST
        +-- 192.168.8.192/26 2 0 2
           |-- 192.168.8.201
              /32 host LOCAL
           |-- 192.168.8.255
              /32 link BROADCAST
Local:
  +-- 0.0.0.0/0 3 0 5
     |-- 0.0.0.0
        /0 universe UNICAST
     +-- 127.0.0.0/8 2 0 2
        +-- 127.0.0.0/31 1 0 0
           |-- 127.0.0.0
              /32 link BROADCAST
              /8 host LOCAL
           |-- 127.0.0.1
              /32 host LOCAL
        |-- 127.255.255.255
           /32 link BROADCAST
     +-- 192.168.8.0/24 2 1 2
        |-- 192.168.8.0
           /32 link BROADCAST
           /24 link UNICAST
        +-- 192.168.8.192/26 2 0 2
           |-- 192.168.8.201
              /32 host LOCAL
           |-- 192.168.8.255
              /32 link BROADCAST";

static UPTIME: &str = "52662.34 619766.90";

static IF_INET6: &str = "fe800000000000000d812a0d8467da1c 04 40 20 80    wlan0
fe800000000000009879b5d0240418bf 01 40 20 80       lo
fe80000000000000f17b7100b5a1f781 05 40 20 80     tun0
00000000000000000000000000000001 01 80 10 80       lo";

static LSPCI: &str =
    "09:00.0 VGA compatible controller: NVIDIA Corporation GK106 [GeForce GTX 660] (rev a1)";

static STOR_DEV: &str = "major minor  #blocks  name
   8       32  312570167 sdc
   8       33  312569127 sdc1
   8       48  117220824 sdd
   8       49     512000 sdd1
   8       50  116707783 sdd2
   8       64  488386584 sde
   8       65     204800 sde1
   8       66   97589248 sde2
   8       67   97590664 sde3
   8       68  292607100 sde4
 254        0  116699136 dm-0";

static STOR_MOUNTS: &str = "proc /proc proc rw,nosuid,nodev,noexec,relatime 0 0
sys /sys sysfs rw,nosuid,nodev,noexec,relatime 0 0
dev /dev devtmpfs rw,nosuid,relatime,size=8149064k,nr_inodes=2037266,mode=755 0 0
run /run tmpfs rw,nosuid,nodev,relatime,mode=755 0 0
efivarfs /sys/firmware/efi/efivars efivarfs rw,nosuid,nodev,noexec,relatime 0 0
/dev/mapper/vgmain-root / ext4 rw,relatime 0 0
securityfs /sys/kernel/security securityfs rw,nosuid,nodev,noexec,relatime 0 0
tmpfs /dev/shm tmpfs rw,nosuid,nodev 0 0
devpts /dev/pts devpts rw,nosuid,noexec,relatime,gid=5,mode=620,ptmxmode=000 0 0
tmpfs /sys/fs/cgroup tmpfs ro,nosuid,nodev,noexec,mode=755 0 0
cgroup2 /sys/fs/cgroup/unified cgroup2 rw,nosuid,nodev,noexec,relatime,nsdelegate 0 0
cgroup /sys/fs/cgroup/systemd cgroup rw,nosuid,nodev,noexec,relatime,xattr,name=systemd 0 0
pstore /sys/fs/pstore pstore rw,nosuid,nodev,noexec,relatime 0 0
none /sys/fs/bpf bpf rw,nosuid,nodev,noexec,relatime,mode=700 0 0
cgroup /sys/fs/cgroup/cpuset cgroup rw,nosuid,nodev,noexec,relatime,cpuset 0 0
cgroup /sys/fs/cgroup/freezer cgroup rw,nosuid,nodev,noexec,relatime,freezer 0 0
cgroup /sys/fs/cgroup/blkio cgroup rw,nosuid,nodev,noexec,relatime,blkio 0 0
cgroup /sys/fs/cgroup/cpu,cpuacct cgroup rw,nosuid,nodev,noexec,relatime,cpu,cpuacct 0 0
cgroup /sys/fs/cgroup/net_cls,net_prio cgroup rw,nosuid,nodev,noexec,relatime,net_cls,net_prio 0 0
cgroup /sys/fs/cgroup/devices cgroup rw,nosuid,nodev,noexec,relatime,devices 0 0
cgroup /sys/fs/cgroup/rdma cgroup rw,nosuid,nodev,noexec,relatime,rdma 0 0
cgroup /sys/fs/cgroup/memory cgroup rw,nosuid,nodev,noexec,relatime,memory 0 0
cgroup /sys/fs/cgroup/perf_event cgroup rw,nosuid,nodev,noexec,relatime,perf_event 0 0
cgroup /sys/fs/cgroup/pids cgroup rw,nosuid,nodev,noexec,relatime,pids 0 0
cgroup /sys/fs/cgroup/hugetlb cgroup rw,nosuid,nodev,noexec,relatime,hugetlb 0 0
systemd-1 /proc/sys/fs/binfmt_misc autofs rw,relatime,fd=29,pgrp=1,timeout=0,minproto=5,maxproto=5,direct,pipe_ino=420 0 0
mqueue /dev/mqueue mqueue rw,nosuid,nodev,noexec,relatime 0 0
hugetlbfs /dev/hugepages hugetlbfs rw,relatime,pagesize=2M 0 0
debugfs /sys/kernel/debug debugfs rw,nosuid,nodev,noexec,relatime 0 0
tracefs /sys/kernel/tracing tracefs rw,nosuid,nodev,noexec,relatime 0 0
tmpfs /tmp tmpfs rw,nosuid,nodev 0 0
configfs /sys/kernel/config configfs rw,nosuid,nodev,noexec,relatime 0 0
/dev/sdd1 /boot vfat rw,relatime,fmask=0022,dmask=0022,codepage=437,iocharset=iso8859-1,shortname=mixed,utf8,errors=remount-ro 0 0
dev /var/lib/dhcpcd/dev devtmpfs rw,nosuid,relatime,size=8149064k,nr_inodes=2037266,mode=755 0 0
proc /var/lib/dhcpcd/proc proc rw,nosuid,nodev,noexec,relatime 0 0
sys /var/lib/dhcpcd/sys sysfs rw,nosuid,nodev,noexec,relatime 0 0
run /var/lib/dhcpcd/run/udev tmpfs rw,nosuid,nodev,relatime,mode=755 0 0
run /var/lib/dhcpcd/run/systemd/journal tmpfs rw,nosuid,nodev,relatime,mode=755 0 0
tmpfs /run/user/1000 tmpfs rw,nosuid,nodev,relatime,size=1632316k,mode=700,uid=1000,gid=1001 0 0
gvfsd-fuse /run/user/1000/gvfs fuse.gvfsd-fuse rw,nosuid,nodev,relatime,user_id=1000,group_id=1001 0 0
fusectl /sys/fs/fuse/connections fusectl rw,nosuid,nodev,noexec,relatime 0 0
fusectl /var/lib/dhcpcd/sys/fs/fuse/connections fusectl rw,nosuid,nodev,noexec,relatime 0 0";

static NET_DEV: &str = "Inter-|   Receive                                                |  Transmit
 face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
     lo:  817348    1992    0    0    0     0          0         0   817348    1992    0    0    0     0       0          0
     enp8s0:       0       0    0    0    0     0          0         0        0       0    0    0    0     0       0          0
      wlan0: 1177144648  929250    0    0    0     0          0         0 59578768  534269    0    0    0     0       0          0
        tun0: 24156600   41069    0    0    0     0          0         0  3623219   43984    0    0    0     0       0          0";

static PROC_STAT: &str = "17008 (htop) S 5868 17008 5868 34823 17008 4194560 557 0 0 0 1169 4309 0 0 20 0 1 0 3236493 11984896 1266 18446744073709551615 94222821875712 94222821963109 140735121616416 0 0 0 0 0 134759430 0 0 0 17 6 0 0 0 0 0 94222822005680 94222822021792 94222838616064 140735121623809 140735121623814 140735121623814 140735121625066 0";

static PROC_STATM: &str = "2926 1266 827 22 0 489 0";

static PROC_CMDLINE: &str = "htop\u{0}-t\u{0}--no-color\u{0}";
