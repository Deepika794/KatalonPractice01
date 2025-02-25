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

WebUI.verifyCheckpoint(findCheckpoint('Checkpoints/Checkpoint'), false)

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.goibibo.com/cars/')

WebUI.setText(findTestObject('Object Repository/Goibibo/Page_Cab Booking - Outstation cabs, Car Ren_73d4f1/input_From_autosuggest'), 
    sourcecity)

WebUI.click(findTestObject('Object Repository/Goibibo/Page_Cab Booking - Outstation cabs, Car Ren_73d4f1/p_Jaipur, Rajasthan, India'))

WebUI.setText(findTestObject('Object Repository/Goibibo/Page_Cab Booking - Outstation cabs, Car Ren_73d4f1/input_To_autosuggest'), 
    destinationcity)

WebUI.click(findTestObject('Object Repository/Goibibo/Page_Cab Booking - Outstation cabs, Car Ren_73d4f1/p_Delhi cantt railway Junction, Kirby Place_a4c4d4'))

WebUI.click(findTestObject('Object Repository/Goibibo/Page_Cab Booking - Outstation cabs, Car Ren_73d4f1/button_SEARCH CABS'))

WebUI.verifyElementPresent(findTestObject('Object Repository/Goibibo/Page_Cab Booking - Outstation cabs, Car Ren_c0e3c8/p_One-way trip  Estimated distance 295 kms _827900'), 
    0)

WebUI.closeBrowser()

