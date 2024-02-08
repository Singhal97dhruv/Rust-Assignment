// Question 2 Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.


fn firstOccurence(arr: &[i32], target: i32)->Option<usize>{
    let mut low=0;
    let mut high=arr.len()-1;
    while low<=high{
        let mid=low+(high-low)/2;
        if arr[mid]==target{
            if mid==0 || arr[mid-1]!=target{
                return Some(mid);
            }else{
                high=mid-1;
            }
            
        }else if arr[mid]<target{
            low=mid+1;
        }else{
            high=mid-1;
        }
    }
    None
}

fn main(){
    let arr=[1,2,2,3,4,5,6,7,7,7,8,9];
    let target=7;
    match firstOccurence(&arr,target){
        Some(index)=>println!("First occurence of {} is at index {} (1-indexed)",target,index),
        None=>println!("{} not found in the array",target),
    }
}