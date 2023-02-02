from math import floor

def part(arr, left, right):
    i = left
    j = right
    piv = arr[floor((left+right)/2)]

    while i <= j:
        while arr[i] < piv:
            i += 1
        while arr[j] > piv:
            j -= 1

        if i <= j :
            arr[i],arr[j] = arr[j],arr[i]
            i += 1
            j -= 1

    return i


def quicksort(arr,left,right):
    if left < right:
        piv = part(arr,left,right)
        quicksort(arr,left,piv-1)
        quicksort(arr,piv, right)


def qsort(arr):
    quicksort(arr,0,len(arr)-1)
    return arr



def main():
    while True:
        arr = list(map(int,input("\n\tenter array, '' to exit: ").split()))
        if arr == []:
            return 0
        print(f"\t {arr} \n\t\t  ↓ \n\t {qsort(arr)}\n")




main()
