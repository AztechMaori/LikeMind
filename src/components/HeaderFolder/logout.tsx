



export default function Logout() {

  async function logoutSession() {
    event?.preventDefault()
    const url = "http://localhost:3000/logout";
    try {
      const res = await fetch(url, {
        method: "GET",
        credentials: "include"
      })
      console.log(`succesfull response:${res}`);
    } catch (err) {
      console.log(`there was an error: ${err}`)

    }

  }

  async function trial() {
    const url = "http://localhost:3000/check";
    try {
      const res = await fetch(url, {
        method: "GET",
        credentials: "include",
      });
      console.log(`succesfull operation ${res.status}`);
    } catch (err) {
      console.log(`there was an error, ${err}`);
    }
  }

  return (
    <>
      <button onClick={logoutSession} class="text-1xl font-bold text-blue-500 hover:text-yellow-500 transition-colors duration-300">
        Logout</button>
      <button onClick={trial}>check for session</button>
    </>
  )
}

