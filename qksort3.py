from math import floor

def partition(arr,left,right):
    piv = arr[floor((left+right)/2)]
    i = left
    j = right
    while i <= j:
        while arr[i] < piv:
            i += 1
        while arr[j] > piv:
            j -= 1
        if i <= j:
            arr[i],arr[j] = arr[j],arr[i]
            i += 1
            j -= 1

    return i


def autoqksort(arr):
    return qksort(arr,0,len(arr)-1)

def qksort(arr,l,r):
    if l < r:
        piv = partition(arr,l,r)
        qksort(arr,l,piv-1)
        qksort(arr,piv,r)

def main():
    while True:
        arr = list(map(int,input('\nenter array: ').split()))
        if arr == []:
            break
        qksort(arr,0,len(arr)-1)
        print(arr)

main()