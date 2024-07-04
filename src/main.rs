fn main() -> Result<(), Box<dyn std::error::Error>> {
    for direntry in std::fs::read_dir("fuzz/corpus/fuzz_target_1")? {
        let path = direntry?.path();
        println!("{}", path.display());
        let data = std::fs::read(path)?;

        let r_patch = schedfuzz::patch::run(&data, 0).map_err(|e| format!("{:?}", e));
        let r_sched = schedfuzz::sched::run(&data, 0).map_err(|e| format!("{:?}", e));
        assert_eq!(r_patch, r_sched);

        let r_patch = schedfuzz::patch::run(&data, 2).map_err(|e| format!("{:?}", e));
        let r_sched = schedfuzz::sched::run(&data, 2).map_err(|e| format!("{:?}", e));
        assert_eq!(r_patch, r_sched);
    }
    Ok(())
}
