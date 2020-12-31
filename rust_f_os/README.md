cd /home/labuser/.cargo/registry/src/github.com-1ecc6299db9ec823/x86_64-0.7.2/  
find . -name "*.rs" | xargs -i sed -i "s,asm,llvm_asm,g" {}  
find . -name "*.rs" | xargs -i sed -i "s,global_llvm_asm,global_asm,g" {}  
cd /home/labuser/.cargo/registry/src/github.com-1ecc6299db9ec823/x86_64-0.7.7/  
find . -name "*.rs" | xargs -i sed -i "s,asm,llvm_asm,g" {}  
find . -name "*.rs" | xargs -i sed -i "s,global_llvm_asm,global_asm,g" {}  
cd /home/labuser/.cargo/registry/src/github.com-1ecc6299db9ec823/bootloader-0.6.4  
find . -name "*.rs" | xargs -i sed -i "s,asm,llvm_asm,g" {}  
find . -name "*.rs" | xargs -i sed -i "s,global_llvm_asm,global_asm,g" {}  
