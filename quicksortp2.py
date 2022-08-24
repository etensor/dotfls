from math import floor

def partition(arr,l,r):
    piv = arr[floor((l+r)/2)]
    i = l
    j = r
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

def quicksort(arr,l,r):
    if l < r:
        piv = partition(arr,l,r)
        quicksort(arr,l,piv-1)
        quicksort(arr,piv,r)
    
    return arr


def prob():
    arr = list(map(int,input("Enter array: ' '").split()))
    if arr == []:
        return None
    else:
        arr = quicksort(arr,0,len(arr)-1)
        print(arr)

prob()