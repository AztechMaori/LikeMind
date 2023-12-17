import {createSignal} from 'solid-js'


 function Counter(){
    const [count, setCount] = createSignal(0);

    function increase(){
        setCount(count()+1)
    }
    return(
        <>
        <div>{count()}</div>
        <button onClick={increase}>increase</button>
        </>
    )
}



export default Counter;

