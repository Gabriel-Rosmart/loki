const result_container = document.getElementById('search-result-container')

document.forms[0].onsubmit = async (e) => {
    e.preventDefault()

    const response = await fetch('http://localhost:8000/', {
        method: 'POST', 
        cache: 'no-cache',         
        headers: {
          'Content-Type': 'text/plain'
        },
        body: document.getElementById('query_input').value
    }).then(response => response.json())
    
    result_container.innerHTML = ''

    for (const row of response) {
      let span = document.createElement('span')
      const splitted = row[0].split('/')
      span.innerText = splitted[splitted.length - 1]; 
      result_container.appendChild(span)
    }
    console.log(response)
}
