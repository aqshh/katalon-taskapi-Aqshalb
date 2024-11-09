<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST Booking</name>
   <tag></tag>
   <elementGuidId>f68f4201-35cc-4f7a-849b-70c719b08c0a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;firstname\&quot;:\&quot;${firstname}\&quot;,\n  \&quot;lastname\&quot;:\&quot;${lastname}\&quot;,\n  \&quot;totalprice\&quot;:\&quot;${totalprice}\&quot;,\n  \&quot;depositpaid\&quot;:\&quot;${depositpaid}\&quot;,\n  \&quot;bookingdates\&quot;:{\n    \&quot;checkin\&quot;:\&quot;${checkin}\&quot;,\n    \&quot;checkout\&quot;:\&quot;${checkout}\&quot;\n  },\n  \&quot;additionalneeds\&quot;:\&quot;${additionalneeds}\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>40634ac2-37c6-4b10-8f6c-4cf354152d03</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>fcdaa113-bdf9-4d11-85da-c2f428ca6a5a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.7.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseURL}/booking</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'Mamang'</defaultValue>
      <description></description>
      <id>b6c7f8d6-e12b-433b-bac6-eb97e41a6b65</id>
      <masked>false</masked>
      <name>firstname</name>
   </variables>
   <variables>
      <defaultValue>'Racing'</defaultValue>
      <description></description>
      <id>6bbf3185-0700-4ce2-90c9-5acf6dee0d86</id>
      <masked>false</masked>
      <name>lastname</name>
   </variables>
   <variables>
      <defaultValue>2500</defaultValue>
      <description></description>
      <id>27debaf1-6adf-476b-9abd-d5eaa1a5d5b3</id>
      <masked>false</masked>
      <name>totalprice</name>
   </variables>
   <variables>
      <defaultValue>true</defaultValue>
      <description></description>
      <id>a1d92e4b-7ee5-4c57-a22d-c20424e0af0d</id>
      <masked>false</masked>
      <name>depositpaid</name>
   </variables>
   <variables>
      <defaultValue>'2024-11-09'</defaultValue>
      <description></description>
      <id>1c9e0fdc-467c-4b48-b35e-b4141cf3776e</id>
      <masked>false</masked>
      <name>checkin</name>
   </variables>
   <variables>
      <defaultValue>'2024-11-14'</defaultValue>
      <description></description>
      <id>c8df06ad-3fd5-48a7-9df3-86f05c552fff</id>
      <masked>false</masked>
      <name>checkout</name>
   </variables>
   <variables>
      <defaultValue>'Race'</defaultValue>
      <description></description>
      <id>51baf6d6-e0fa-4d8b-8870-08bdd72c712b</id>
      <masked>false</masked>
      <name>additionalneeds</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
