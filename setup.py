# -*- coding: utf-8 -*-
from setuptools import setup, find_packages
from setuptools_rust import Binding, RustExtension


setup(
    name='q5',
    version='0.0.3',
    long_description=open('README.md').read(),
    long_description_content_type='text/markdown',
    packages=find_packages(),
    zip_safe=False,
    include_package_data=True,
    rust_extensions=[
        RustExtension(
            'q5.engine',
            path='Cargo.toml',
            binding=Binding.PyO3,
            debug=False,
        )
    ],
)
