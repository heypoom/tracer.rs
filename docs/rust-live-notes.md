# Agenda

1. Introduction
- Self Intro

2. Tracer.rs: Project and Scope
- Tracer.rs is a daemon (server) for system call exfiltration.
  - Why? What's the Problem?
  - Why do we need to analyze system calls?
  - Existing Tools: STrace and PTrace
- Getting trace results from the Kernel
  - Client should be able to subscribe to the tracer daemon
- Filtering/Matching the system calls to get what we want
- Multi-Threaded
  - Collector Thread -> Collect results from PTrace
  - Consumer Thread  -> Send result to the client for visualization

3. Why use Rust?
- Should be a single binary for easy installation
  - e.g. `curl https://tracer.rs/run.sh | bash`
- Install as a low-overhead daemon in your server or k8s pod.
- Rust can inject into the ptrace syscall with zero cost.
  - C FFI -> zero cost.
- Low Overhead -> Memory and file size (binary)
  - LLVM -> 200k stripped

4. Time to Hack!

# Checklist

1. Getting trace results from the Kernel
- Using Rust matchers to filter the results

2. Exfiltrating to external systems with multi-threading.
- Collector Thread.
- Consumer Thread.
- TCP Server? (For sending results to client)

