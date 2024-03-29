# Zappi Metrics Exporter
This is a Prometheus exporter for zappi api


## Other Dependencies
OpenSSL is required as part of the dependencies 
### Ubuntu
`apt install pkg-config libssl-dev`

### Fedora
`dnf install openssl-devel`

## API Research
This gives us the basic data used on the app's main screen.   The app also makes calls to 

  `/cgi-jstatus-Z`  (For Zappi data)
  
```json
{
	"zappi": [{
		"che": 1,			//Charge added in KWh
		"cmt": 253,		//Command Timer- counts 1 - 10 when command sent, then 254 - success, 253 - failure, 255 - never received any comamnds
		"dat": "07-06-2019",	//Date	
		"div": 1376,		//Diversion amount Watts (does not appear if zero)
		"dst": 1,			// Use Daylight Savings Time
		"ectp1": 920,		//Physical CT connection 1 value Watts
		"ectp2": 2143,		//Physical CT connection 2 value Watts
		"ectp3": 2143,		//Physical CT connection 3 value Watts
		"ectp4": 2143,		//Physical CT connection 4 value Watts
		"ectp5": 2143,		//Physical CT connection 5 value Watts
		"ectp6": 2143,		//Physical CT connection 6 value Watts
		"ectt1": "Grid",		//CT 1 Name	
		"ectt2": "Generation",	//CT 2 Name	
		"ectt3": " ",		//CT 3 Name
		"ectt4": " ",		//CT 4 Name
		"frq": 49.95,		//Supply Frequency
		"fwv": 1234		//Firmware Version
		"gen": 2143,		//Generated Watts
		"grd": 1017,		//Watts from grid?
		"lck": 10,		//Lock Status (4 bits : 1st digit - ? : 2nd digit - 1 unlocked, 0 locked)
		"mgl": 100,		//Minimum Green Level
		"pha": 1,			// Phases
		"pri": 1,			//priority
		"pst": "A",		//Status A=EV Disconnected, B1=EV Connected, B2=Waiting for EV, C1=EV Ready to Charge, C2= Charging, F= Fault
		"sbh": 14,		//Smart Boost Start Time Hour
		"sbk": 5			//Smart Boost KWh to add
		"sbm": 15,		//Smart Boost Start Time Minute
		"sno": 10077777,        	//Changed Zappi Serial Number		
		"sta": 3,			//Status  1=Paused 3=Diverting/Charging 5=Complete
		"tbh": 9,			//boost hour?
		"tbk": 90,		//boost KWh   - Note charge remaining for boost = tbk-che
		"tbm": 15,		//boost minute?
		"tim": "07:28:46",	//Time	
		"vol": 244.4,		//Supply voltage
		"zmo": 3,			//Zappi Mode - 1=Fast, 2=Eco, 3=Eco+, 4=Stopped
	}]
}
```

Lock Status

'lck' - representation of current PIN lock settings and zappi lock status
Bit 0: Locked Now
Bit 1: Lock when plugged in
Bit 2: Lock when unplugged.
Bit 3: Charge when locked.
Bit 4: Charge Session Allowed (Even if locked)

Note that charging will be allowed if:-
Locked Now=0
or
Locked Now=1 and Charge When Locked=1
or
Charge Session Allowed=1

----

This repo was inspired from [tado-exporter](https://github.com/eko/tado-exporter). Please check it out if you have a tado setup and want better metrics.