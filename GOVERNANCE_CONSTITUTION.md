# **Governance Constitution**

## **Article I. The Council**

### **Section 1\. Composition**

1. The Council shall consist of a minimum of ten (10) members.  
2. Members are elected via a global community vote. Candidacy is limited solely to persons who have not previously served their allowed terms and accounts that are not on the Global Revocation List (Blacklist).  
3. Council seats are intended to be regionally distributed to ensure global perspective, though this is enforced by voter choice, not hard quotas.

### **Section 2\. Terms**

1. A Council term lasts for four (4) years.  
2. No individual may serve more than two (2) terms, consecutive or non-consecutive.

### **Section 3\. Powers**

The Council is granted the following specific powers:

1. Ratification of software updates, forks, and protocol changes.  
2. Proposal and revocation of Supplemental Guidelines for community behavior.  
3. Maintenance of the Global Revocation List (Blacklist) for accounts that violate the Global Guidelines (spam, egregious abuse).

## **Article II. Voting and Quorum**

### **Section 1\. Operational Quorum**

1. A valid Council vote requires a quorum of 66% of seated members.  
2. Decisions are carried by a simple majority (\>50%) of the quorum, except for Constitutional Amendments which require a supermajority (75%).

### **Section 2\. Threshold Logging**

1. All official Council actions must be signed using the Council's active Threshold Signature.  
2. Any action appearing on-chain without a valid Threshold Signature is null and void and must be rejected by Clients.

## **Article III. Special Governance States**

### **Section 1\. Automatic Dissolution**

The Council is automatically dissolved if:

1. It fails to reach quorum.  
2. The number of active members falls below the constitutional minimum of ten (10).

### **Section 2\. Vote of No Confidence**

1. The Community may initiate a Vote of No Confidence, provided that at least thirty (90) days have passed since the conclusion of any previous Vote of No Confidence.  
2. Once initiated, the vote shall remain open for a period of fourteen (14) days to gather necessary support.  
3. If a simple majority of all community members who have been active within the past 3 months votes for dissolution by the end of this period, the Council is immediately dissolved.

### **Section 3\. Caretaker Mode**

Upon dissolution, the protocol enters "Caretaker Mode":

1. No new policy changes or blacklists can be issued.  
2. The Runtime automatically schedules a new general election.

### **Section 4\. Bootstrap Phase**

1. The Protocol includes a "Bootstrap Phase" to ensure sufficient decentralization before governance activation.  
2. Council formation and voting provisions shall remain suspended until the network reaches a minimum threshold of **1,000 Active Unique Identities** (this value was determined based on an application of Cochran’s Formula to account for representative sampling where \~385 users provide a 95% confidence level for an infinite population, and \~664 provide 99% confidence). "Active" is defined as having committed at least one on-chain transaction within the last 90 days.  
3. To ensure resilience, these identities must be distributed across at least **three (3) distinct geographic regions** (e.g., North America, Europe, Asia-Pacific).  
4. Furthermore, the network must verify that a simple majority (\>50%) of these active identities originate from jurisdictions designated as **"Free"** in the most recent annual **"Freedom on the Net"** report published by **Freedom House** (or an equivalent widely-recognized international index ratified by the Council).  
5. Prior to meeting these thresholds, the protocol operates under genesis rules. Upon verification of these criteria by the Runtime, the first General Election is automatically scheduled.

## **Article IV. Forks and Ratification**

### **Section 1\. Legitimacy**

1. A "Legitimate Fork" is defined as a modification to the Platform Code that has been cryptographically signed and ratified by the Council.  
2. Client software shall be programmed to reject connection to networks running unratified forks.

### **Section 2\. Override**

1. If a Council refuses to ratify a fork desired by the majority, the Community may vote to bypass the Council.  
2. A Community Ratification vote (\>50%) supersedes a Council veto.

## **Article V. Amendments**

This Constitution may only be amended by a proposal ratified by a 75% supermajority of the Council AND a majority ratification by the Community.