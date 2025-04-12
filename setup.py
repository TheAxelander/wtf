from setuptools import setup

setup(
    name='wtf',
    version='0.1',
    packages=['wtf'],
    install_requires=[
        'rich'
    ],
    entry_points={
        'console_scripts': [
            'wtf=wtf.main:main'
        ],
    },
    url='',
    license='MIT',
    author='Alexander Preibisch',
    author_email='alexpreib@outlook.com',
    description=''
)
