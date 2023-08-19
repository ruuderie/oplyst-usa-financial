module.exports = {
  apps: [
    {
      name: 'oplyst-international-llc-loan-landscape',
      port: '8081',
      exec_mode: 'cluster',
      instances: 'max',
      script: './.output/server/index.mjs'
    }
  ]
}