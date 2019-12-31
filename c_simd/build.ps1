
# see https://github.com/mverleg/jvm_try/tree/master/tryjni

# run --rm --mount 'type=bind,dst=/code,src=C:\Code\QIS\cea\QuinityFormsAdministration\resources\nativec' --mount 'type=bind,dst=/code/com/keylane/nativec/c,src=C:\Code\QIS\cea\AxonNativeCExecution\resources\com\keylane\nativec\c' -it i-gcc bash -c 'ls; ls com/keylane/nativec/c/'

# some compilers need this: -D__int64=int64_t `
# -I"C:\Code\QIS\cea\AxonNativeCExecution\resources" `
# somehow "-fstack-protector-strong" gave a load error from java due to dependency 'libssp_64-0.dll'

#if ( ! ("$env:INCLUDE_PATH" -match "nativec") ) {
#    $env:INCLUDE_PATH += ";$(Resolve-Path -Path ..\..\..\..\nativec\resources)"
#    echo "updating INCLUDE_PATH to $env:INCLUDE_PATH"
#}

function compile_c_simd {
    param(
        [Parameter(Mandatory=$false)][Alias('opt')][switch]$optimize,
        [Parameter(Mandatory=$false)][Alias('dump')][switch]$dodump
    )

    $cmd="gcc -fargument-noalias-global -Wall -Wextra -pedantic " +
            "-Werror -Wconversion -fno-builtin-log -Wno-unused-variable " +
            "-Wno-unused-but-set-variable -Wno-unused-function " +
            "-Wno-unused-parameter -std=gnu11 -x c -march=native -mtune=native " +
            "-msse -msse2 -msse3 -mfpmath=sse " +
            "-ffast-math -fno-trapping-math -fno-math-errno -D__int64=int64_t "
    if ($optimize.IsPresent) {
        echo "optimized build"
        $cmd += "-pipe -O3 -fomit-frame-pointer -ftree-vectorize -flto "
        if ($dodump.IsPresent) {
            $cmd += "-g "
        }
    } else {
        echo "debug build"
        $cmd += "-Ddebug -Dtiming -fbounds-check -ftrapv -O0 -g -p " +
            "-pg -grecord-gcc-switches -D_FORTIFY_SOURCE=2 "
    }
    $cmd += "-I`"${env:JAVA_HOME}\include`" -I`"${env:JAVA_HOME}\include\win32`" " +
            "-o csimd.exe " +
            "main.c " +
            "-static -lpthread -fopenmp "
    echo $cmd

    $time = Measure-Command { Invoke-Expression "$cmd" }

    echo "compile done (${time})"

    if ($dodump.IsPresent) {
        objdump -S -l csimd.exe > objdump.asm
        echo "objdump done"
    }
    echo ""
}

compile_c_simd -opt -dump

