
const func = async () => {

    await window.deskmsg.startNativeServer()
    console.log('get config:', await window.deskmsg.getNativeServerConfig())

}
console.log('render script run')
setTimeout(func, 5000)
