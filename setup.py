# -*- coding: utf-8 -*-
from setuptools import setup, find_namespace_packages
from setuptools_rust import Binding, RustExtension


setup(
    name='q5',
    version='0.0.1',
    packages=find_namespace_packages(include=['q5.*']),
    zip_safe=False,
    rust_extensions=[
        RustExtension(
            'q5.engine',
            path='Cargo.toml',
            binding=Binding.PyO3,
            debug=False,
        )
    ],
)
