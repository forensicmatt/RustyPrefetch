import os
import sys
import shutil
import pkgutil
import platform
from setuptools import setup
from setuptools_rust import RustExtension

def post_build():
    platform_str = platform.system()
    if platform_str == 'Windows':
        dylib_name = 'libfwnt.dll'
    elif platform_str == 'Linux':
        dylib_name = 'libfwnt.so'
    else:
        raise(Exception("{} platform is not handled.".format(platform_str)))

    flag_64x = sys.maxsize > 2**32
    if flag_64x:
        dylib_fullname = os.path.join(
            os.environ['LIBFWNT_BIN64'],
            dylib_name
        )
    else:
        dylib_fullname = os.path.join(
            os.environ['LIBFWNT_BIN32'],
            dylib_name
        )

    if not os.path.isfile(dylib_fullname):
        dylib_fullname = os.path.join(
            os.environ['LIBFWNT_BIN'],
            dylib_name
        )
        if not os.path.isfile(dylib_fullname):
            raise(Exception("{} not found.".format(dylib_fullname)))

    print("found {}".format(dylib_fullname))
    package_dir = os.path.dirname(pkgutil.get_loader("pyrpf").filename)

    print("found {}".format(package_dir))
    dylib_newname = os.path.join(
        package_dir,
        dylib_name
    )

    shutil.copyfile(
        dylib_fullname,
        dylib_newname
    )

setup(
    name='pyrpf',
    version='0.1.0',
    rust_extensions=[
        RustExtension('pyrpf', 'Cargo.toml')
    ],
    zip_safe=False
)

post_build()
