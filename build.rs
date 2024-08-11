fn main(){
    #[cfg(feature = "cuda-support")]
    {
        let build_list = ["lib"];
        let component_build_list = ["vector"];

        for build in build_list {
            cc::Build::new()
                .file(format!("src/cuda/{}.cu", build))
                .cuda(true)
                .compile(build);
        }

        for build in component_build_list {
            #[cfg(feature = "rough-num")]
            cc::Build::new()
                .file(format!("src/cuda/f32/{}.cu", build))
                .cuda(true)
                .compile(&format!("{}_f32", build));
            #[cfg(not(feature = "rough-num"))]
            cc::Build::new()
                .file(format!("src/cuda/f64/{}.cu", build))
                .cuda(true)
                .compile(&format!("{}_f64", build));
        }
    }
}
