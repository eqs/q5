# -*- coding: utf-8 -*-
from setuptools import setup, find_packages
from setuptools_rust import Binding, RustExtension


setup(
    name='q5',
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
        ),
        RustExtension(
            'q5.numpy_lib',
            path='q5_numpy/Cargo.toml',
            binding=Binding.PyO3,
            debug=False,
        )
    ],
    license='MIT',
    python_requires='>=3.8',
    classifiers=[
        'Intended Audience :: Education',
        'License :: OSI Approved :: MIT License',
        'Programming Language :: Python :: 3',
        'Topic :: Multimedia :: Graphics',
        'Topic :: Multimedia :: Sound/Audio'
    ],
    setup_requires=[
        "setuptools_scm"
    ]
)
