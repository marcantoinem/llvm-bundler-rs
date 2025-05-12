mkdir -p .llvm
cd .llvm

#rm -fr llvm
mkdir -p llvm
rm -fr llvm-project
git clone --depth 1 https://github.com/llvm/llvm-project.git
cd llvm-project
git fetch origin llvmorg-20.1.4
git checkout llvmorg-20.1.4

#cmake -S llvm -B build -G Ninja \
#  -DCMAKE_BUILD_TYPE=Release \
#  -DLLVM_ENABLE_PROJECTS="clang;mlir" \
#  -DLLVM_TARGETS_TO_BUILD="X86" \
#  -DLLVM_BUILD_TESTS=OFF \
#  -DLLVM_INCLUDE_TESTS=OFF \
#  -DLLVM_BUILD_EXAMPLES=OFF \
#  -DLLVM_INCLUDE_EXAMPLES=OFF \
#  -DLLVM_BUILD_DOCS=OFF \
#  -DLLVM_ENABLE_DOXYGEN=OFF \
#  -DLLVM_ENABLE_LTO=OFF \
#  -DLLVM_ENABLE_SPHINX=OFF \
#  -DLLVM_STATIC_LINK_CXX_STDLIB=ON \
#  -DLLVM_ENABLE_ZLIB=OFF \
#  -DLLVM_ENABLE_LIBXML2=OFF \
#  -DLLVM_ENABLE_LIBEDIT=OFF \
#  -DCMAKE_INSTALL_PREFIX=../llvm
#
#ninja -C build install
