type Props = {
    input: string,
    isDown: boolean,
}

export default function Input({input, isDown}: Props){

    return isDown ? <div className='flex bg-slate-500 w-8 h-8 rounded-lg shadow m-0.5 items-center justify-center'>
            <span className="text-black">{input}</span>
        </div> : <div className='flex bg-white w-8 h-8 rounded-lg shadow m-0.5 items-center justify-center'>
            <span className="text-black">{input}</span>
        </div>
}