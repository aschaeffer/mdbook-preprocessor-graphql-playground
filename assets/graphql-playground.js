const DEFAULT_ENDPOINT = 'http://localhost:31415/graphql'

function createGraphQLPlaygroundInstance(container, endpoint, tabs) {
  if (typeof GraphQLPlayground !== 'undefined') {
    delete GraphQLPlayground
  }
  const script = document.createElement('script')
  script.src = '/graphql-playground-react-middleware.js'
  script.onload = function () {
    const GraphQLPlaygroundInstance = GraphQLPlayground
    GraphQLPlaygroundInstance.init(
      container,
      {
        endpoint,
        tabs
      }
    )
  };
  document.documentElement.firstChild.appendChild(script);
}

function createHeader(parent, title, description) {
  const header = document.createElement('div')
  header.classList.add('graphql-playground-header')
  const heading = document.createElement('h3')
  heading.classList.add('graphql-playground-heading')
  heading.classList.add('admonition')
  heading.classList.add('info')
  heading.innerHTML = title
  heading.title = `GraphQL Example "${title}": Click to open in fullscreen!`
  heading.addEventListener('click', function () {
    if (document.fullscreenElement) {
      document.exitFullscreen()
    } else {
      container.requestFullscreen()
    }
  })
  const descriptionNode = document.createElement('div')
  descriptionNode.classList.add('graphql-playground-description')
  descriptionNode.classList.add('admonition')
  descriptionNode.classList.add('info')
  const descriptionInner = document.createElement('div')
  descriptionInner.innerHTML = `<p>${description}</p>`
  descriptionNode.append(descriptionInner)
  header.append(heading, descriptionNode)
  return header
}

function createInstanceContainer(id) {
  const container = document.createElement('graphql-playground-container')
  container.id = `${id}-playground`
  return container
}

function createInstance(container, config) {
  container.innerHTML = ''
  const instanceContainer = createInstanceContainer(container.id)
  const header = createHeader(container, config.title, config.description)
  container.append(header, instanceContainer)
  return instanceContainer
}

function fetchQuery(url, endpoint) {
  endpoint = endpoint || DEFAULT_ENDPOINT
  console.log('fetchQuery', name, url, endpoint)
  return fetch(url).then((response) => response.text())
}

function getTabConfig(name, url, endpoint) {
  endpoint = endpoint || DEFAULT_ENDPOINT
  console.log('getTabConfig', name, url, endpoint)
  return fetchQuery(url, endpoint).then(query => {
    return {
      endpoint,
      name,
      query
    }
  })
}

function getTabConfigs(config, endpoint) {
  endpoint = endpoint || DEFAULT_ENDPOINT
  console.log('getTabConfigs', config, endpoint)
  return Promise.all(
    config.tabs.map(tab => getTabConfig(tab.name, tab.url, endpoint))
  )
}

function getContainerAndConfig(id) {
  const container = document.getElementById(id)
  const config = JSON.parse(container.innerHTML)
  return {
    container,
    config
  }
}

window.GraphQLPlaygroundInstance = function (id, endpoint) {
  endpoint = endpoint || DEFAULT_ENDPOINT
  const { container, config } = getContainerAndConfig(id)
  console.log('GraphQLPlaygroundInstance', id, endpoint, container, config)
  const instanceContainer = createInstance(container, config)
  getTabConfigs(config).then(tabs => {
    console.log(tabs)
    createGraphQLPlaygroundInstance(
      instanceContainer,
      endpoint,
      tabs
    )
  })
}

window.addEventListener('load', function () {
  const playgrounds =  Array.from(document.getElementsByTagName('graphql-playground'))
  for (let playground of playgrounds) {
    window.GraphQLPlaygroundInstance(playground.id)
  }
})
