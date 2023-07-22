# OVH DNS Updater

### Update OVH DNS records with your public IPv4 address and/or IPv6 address

---

This program aims to update your OVH DNS records with your public IP address. It can update both IPv4 and IPv6 addresses.
It is useful if you have a dynamic IP address and want to access your home network from outside.

This is suited for a cron job, for example every 5 minutes.

### Note:
You can also use DynDNS, but you would be more limited in the functionalities you can have.

---

## How it works

* First it retrieves simultaneously your public IPv4 and/or IPv6 address from the websites:
  * https://api.ipify.org / https://api6.ipify.org
  * https://ipv4.lafibre.info / https://ipv6.lafibre.info
  * https://v4.ident.me/ / https://v6.ident.me/
  * https://ip4.me/ / https://ip6only.me/
* Then program takes the IP address from the first website that responds.
* Meanwhile, it retrieves the current IP address of the DNS record from the OVH API (you have to make sure the record exists).
* If the IP address from the website is different from the one from the DNS record, it updates the DNS record with the new IP address.

## Usage

### Records configuration:

Make sure you have a DNS record configured on your OVH account. This program doesn't create records, it only updates them.

### Token creation:

First create tokens for your application. You can do this by going to the [OVH API token creation page](https://www.ovh.com/auth/api/createToken) (https://www.ovh.com/auth/api/createToken).

You will need to select the following rights:
* __GET /domain/zone/\*__
* __PUT /domain/zone/\*__

![Token creation](/assets/ovh_api_create_credentials.png)


If you want to revoke the token, you can do it by using the API call DELETE /me/api/application/{applicationId} (https://api.ovh.com/console/#/me/api/application/%7BapplicationId%7D~DELETE)


### Installation:

```bash
cargo build --release
```

### Setting token credentials:

You have to __set credentials as environment variables__. The most convenient way to do this is to create a `.env` file in the root of the project and set the variables there (__Don't forget to set correct permissions on the `.env` file__).
```dotenv
OVH_ENDPOINT=[Domain zone endpoint on which you want to update records]
OVH_APPLICATION_KEY=****************
OVH_APPLICATION_SECRET=********************************
OVH_CONSUMER_KEY=********************************
```

Supported endpoints:
* ovh-eu
* ovh-ca
* ovh-us
* kimsufi-eu
* kimsufi-ca
* soyoustart-eu
* soyoustart-ca

Note that you can opt for a more secure way to store your credentials, for example by using [HashiCorp Vault](https://www.vaultproject.io/).

### Running:

```
Usage: ovh_dns_updater.exe [OPTIONS] --record <RECORD>

Options:
  -n, --no-ip4           
  -6, --upgrade-ip6      
  -r, --record <RECORD>
  -h, --help             Print help
  -V, --version          Print version
```

* no-ip4: Don't update IPv4 address
* upgrade-ip6: Update IPv6 address
* record: The DNS record to update, for example: `mydomain.com` or `subdomain.mydomain.com`

---

Thanks https://github.com/MicroJoe/rust-ovh

