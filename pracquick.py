from math import floor

def part(arr,left,right):
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


def quicksort(arr,left,right):
    if left < right:
        piv = part(arr,left,right)
        quicksort(arr,left,piv-1)
        quicksort(arr,piv,right)



def main():
    while True:
        arr = list(map(int,input('\nEnter array\n\t<== ').split()))
        if arr  == []:
            return 0
        quicksort(arr,0,len(arr)-1)
        print('\t==> ',arr)
        

main()