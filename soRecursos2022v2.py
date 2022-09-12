#IMPORTAMOS LIBRERIAS.
import psutil
import platform
from datetime import datetime
import os
from time import sleep

#FUNCION PARA REPRESENTAR TAMAÑOS EN BITS.
def get_size(bytes, suffix="B"):
    factor = 1024
    for unit in ["", "K", "M", "G", "T", "P"]:
        if bytes < factor:
            return f"{bytes:.2f}{unit}{suffix}"
        bytes /= factor

def get_system_info():    
    
    os.system('cls' if os.name == 'nt' else 'clear')

    #INFORMACIÓN BÁSICA DEL SISTEMA.
    print("="*40, "System Information", "="*40)
    uname = platform.uname()
    print(f"System: {uname.system}")
    print(f"Node Name: {uname.node}")
    print(f"Release: {uname.release}")
    print(f"Version: {uname.version}")
    print(f"Machine: {uname.machine}")
    print(f"Processor: {uname.processor}")

    #INFORMACIÓN DE LA CPU
    print("="*40, "CPU Info", "="*40)
    #Nº NUCLEOS
    print("Physical cores:", psutil.cpu_count(logical=False))
    print("Total cores:", psutil.cpu_count(logical=True))
    #FRECUENCIAS CPU
    cpufreq = psutil.cpu_freq()
    print(f"Max Frequency: {cpufreq.max:.2f}Mhz")
    print(f"Min Frequency: {cpufreq.min:.2f}Mhz")
    print(f"Current Frequency: {cpufreq.current:.2f}Mhz")
    #USO DE CPU
    print("CPU Usage Per Core:")
    for i, percentage in enumerate(psutil.cpu_percent(percpu=True)):
        print(f"Core {i}: {percentage}%")
    print(f"Total CPU Usage: {psutil.cpu_percent()}%")
    #MEMORIA
    print("="*40, "Memory Information", "="*40)
    svmem = psutil.virtual_memory()
    print(f"Total: {get_size(svmem.total)}")
    print(f"Available: {get_size(svmem.available)}")
    print(f"Used: {get_size(svmem.used)}")
    print(f"Percentage: {svmem.percent}%")

    if(svmem.percent >= 40):
        print("*******Alerta de Memoria ***********")

    print("="*20, "SWAP", "="*20)
    swap = psutil.swap_memory()
    print(f"Total: {get_size(swap.total)}")
    print(f"Free: {get_size(swap.free)}")
    print(f"Used: {get_size(swap.used)}")
    print(f"Percentage: {swap.percent}%")
    sleep(1.2)


while True:
    get_system_info()