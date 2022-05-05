import math

# operacion in place: no copy 
# pass by reference

def merge(p,q,r,A):
    left = A[p:q+1]      # incluir a q
    right = A[q+1:r+1]   # incluir a r

    left.append(float('inf'))
    right.append(float('inf'))
    i = j = 0
    for k in range(p,r+1):
        if left[i] <= right[j]:
            A[k] = left[i]
            i += 1
        else:
            A[k] = right[j]
            j += 1


def mergeSort(p,r,A):
    print(p,r)
    if p < r:
        q = math.floor((p+r)/2)
        mergeSort(p,q,A)
        mergeSort(q+1,r,A)
        merge(p,q,r,A)


lista = [2,10,3,12,20,30,-1,13,27]
mergeSort(0,len(lista)-1,lista)
print('\n\n',lista)