flags = [
    # Define your per project flags here, flags starting with '.' will be expanded

    # Language flags
    '-xc',
    '-std=gnu89',
    # Warning flags
    '-Wall',
    '-Wundef',
    '-Wstrict-prototypes',
    '-Wno-trigraphs',
    '-fno-strict-aliasing',
    '-fno-common',
    '-Werror-implicit-function-declaration',
    '-Wno-format-security',
    '-Wdeclaration-after-statement',
]

def FlagsForFile(filename):
    return {
        'flags': flags,
        'do_cache': True
    }
