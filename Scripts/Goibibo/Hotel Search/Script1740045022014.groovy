import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.makemytrip.com/hotels/')

WebUI.click(findTestObject('Object Repository/Makemytrip/Page_MakeMyTrip.com Save upto 60 on Hotel B_affcea/span_ENG_commonModal__close'))

WebUI.click(findTestObject('Object Repository/Makemytrip/Page_MakeMyTrip.com Save upto 60 on Hotel B_affcea/input_City, Property name or Location_city'))

WebUI.click(findTestObject('Object Repository/Makemytrip/Page_MakeMyTrip.com Save upto 60 on Hotel B_affcea/a_CLEAR'))

WebUI.click(findTestObject('Object Repository/Makemytrip/Page_MakeMyTrip.com Save upto 60 on Hotel B_affcea/p_Delhi'))

WebUI.click(findTestObject('Object Repository/Makemytrip/Page_MakeMyTrip.com Save upto 60 on Hotel B_affcea/div_25'))

WebUI.click(findTestObject('Object Repository/Makemytrip/Page_MakeMyTrip.com Save upto 60 on Hotel B_affcea/div_28'))

WebUI.click(findTestObject('Object Repository/Makemytrip/Page_MakeMyTrip.com Save upto 60 on Hotel B_affcea/button_Apply'))

WebUI.click(findTestObject('Object Repository/Makemytrip/Page_MakeMyTrip.com Save upto 60 on Hotel B_affcea/button_Search'))

WebUI.click(findTestObject('Object Repository/Makemytrip/Page_Delhi Hotels, resorts, homestays and m_f877ca/p_Showing Properties in Delhi'))

WebUI.verifyElementText(findTestObject('Object Repository/Makemytrip/Page_Delhi Hotels, resorts, homestays and m_f877ca/p_Showing Properties in Delhi'), 
    'Showing Properties in Delhi')

WebUI.closeBrowser()

