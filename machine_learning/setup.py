from setuptools import setup, find_packages

setup(
    name="news",
    version="0.1.0",
    packages=find_packages(include=["rest", "rest.*"]),
)
