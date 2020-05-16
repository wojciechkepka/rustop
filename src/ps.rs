#![allow(dead_code)]
use super::*;
use std::path::PathBuf;

// Remember to use libc::_SC_CLK_TCK to measure CPU time
// and libc::_SC_PAGE_SIZE for memory

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum ProcessState {
    Running,
    Sleeping,
    Waiting,
    Zombie,
    Stopped,
    TracingStop,
    Dead,
    Wakekill,
    Waking,
    Parked,
    Unknown,
}
impl From<char> for ProcessState {
    fn from(c: char) -> ProcessState {
        use self::ProcessState::*;
        match c {
            'R' => Running,
            'S' => Sleeping,
            'D' => Waiting,
            'Z' => Zombie,
            'T' => Stopped,
            't' => TracingStop,
            'X' | 'x' => Dead,
            'K' => Wakekill,
            'W' => Waking,
            'P' => Parked,
            _ => Unknown,
        }
    }
}
impl Default for ProcessState {
    fn default() -> ProcessState {
        ProcessState::Unknown
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Eq, PartialEq)]
pub struct Process {
    /// The process ID.
    pub pid: u32,
    /// Binary application name of this process
    pub name: String,
    /// Full command with arguments
    pub cmd: String,
    /// The process state
    pub state: ProcessState,
    /// The PID of the parent of this process.
    pub ppid: u32,
    /// The process group ID of the process.
    pub pgrp: u32,
    /// Amount of time that this process has been scheduled
    /// in user mode, measured in clock ticks.
    pub utime: u64,
    /// Amount of time that this process has been scheduled
    /// in kernel mode, measured in clock ticks.
    pub stime: u64,
    /// total program size
    pub size: u64,
    /// resident set size
    pub resident: u64,
    /// number of resident shared pages (i.e., backed by a file)
    pub shared: u64,
    pub uid: u32,
    pub gid: u32,
}
impl Process {
    pub fn new(pid: u32) -> Result<Process> {
        let mut proc = Process::default();
        let p = PathBuf::from(format!("/proc/{}", pid));
        let stats = fs::read_to_string(p.join("stat"))?;
        proc.parse_proc_stat(&stats)?;
        let statsm = fs::read_to_string(p.join("statm"))?;
        proc.parse_proc_statm(&statsm)?;
        let (uid, gid) = Self::uid_gid(pid)?;
        proc.uid = uid;
        proc.gid = gid;
        proc.cmd = Self::cmd(pid)?;

        Ok(proc)
    }

    /// Re-reads /proc/[pid]/stat file and updates struct fields
    fn update_stat(&mut self) -> Result<()> {
        let p = PathBuf::from(format!("/proc/{}", self.pid));
        let stats = fs::read_to_string(p.join("stat"))?;
        self.parse_proc_stat(&stats)
    }

    /// Internal function to parse out interesting attributes
    /// about a process from /self/[pid]/stat
    pub(crate) fn parse_proc_stat(&mut self, out: &str) -> Result<()> {
        let mut attrs = out.split(' ');
        if let Some(pid) = attrs.next() {
            self.pid = pid.parse::<u32>()?;
        }
        if let Some(name) = attrs.next() {
            self.name = name
                .trim_start_matches('(')
                .trim_end_matches(')')
                .to_string();
        }
        if let Some(state) = attrs.next() {
            self.state = ProcessState::from(state.chars().next().unwrap())
        }
        if let Some(ppid) = attrs.next() {
            self.ppid = ppid.parse::<u32>()?;
        }
        if let Some(pgrp) = attrs.next() {
            self.pgrp = pgrp.parse::<u32>()?;
        }
        let mut attrs2 = attrs.skip(8);
        if let Some(utime) = attrs2.next() {
            self.utime = utime.parse::<u64>()?;
        }
        if let Some(stime) = attrs2.next() {
            self.stime = stime.parse::<u64>()?;
        }

        Ok(())
    }

    fn update_statm(&mut self) -> Result<()> {
        let p = PathBuf::from(format!("/proc/{}", self.pid));
        let stats = fs::read_to_string(p.join("statm"))?;
        self.parse_proc_statm(&stats)
    }

    /// Internal function to parse out interesting attributes
    /// about process memory from /self/[pid]/statm
    pub(crate) fn parse_proc_statm(&mut self, out: &str) -> Result<()> {
        let mut attrs = out.split(' ');
        if let Some(size) = attrs.next() {
            self.size = size.parse::<u64>()?;
        }
        if let Some(resident) = attrs.next() {
            self.resident = resident.parse::<u64>()?;
        }
        if let Some(shared) = attrs.next() {
            self.shared = shared.parse::<u64>()?;
        }

        Ok(())
    }

    /// Returns (uid, gid) of process
    pub(crate) fn uid_gid(pid: u32) -> Result<(u32, u32)> {
        let p = PathBuf::from(format!("/proc/{}", pid));
        let status = fs::read_to_string(p.join("status"))?;
        let mut uid = 0;
        let mut gid = 0;
        let uid_re = Regex::new(r"Uid:\s+(\d+)").unwrap();
        let gid_re = Regex::new(r"Gid:\s+(\d+)").unwrap();
        for m in uid_re.captures_iter(&status) {
            uid = m[1].parse::<u32>()?;
        }
        for m in gid_re.captures_iter(&status) {
            gid = m[1].parse::<u32>()?;
        }
        Ok((uid, gid))
    }

    /// Returns a full command line of process
    pub fn cmd(pid: u32) -> Result<String> {
        let p = PathBuf::from(format!("/proc/{}", pid));
        match fs::read_to_string(p.join("cmdline")) {
            Ok(out) => Ok(Self::_cmd(&out)),
            Err(e) => Err(anyhow!("{}", e)),
        }
    }
    pub(crate) fn _cmd(out: &str) -> String {
        out.trim_end_matches('\u{0}').replace('\u{0}', &" ")
    }
}
