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

WebUI.navigateToUrl('https://www.practo.com/')

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo  Video Consultation with Doctor_95fe55/a_Login  Signup'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/a_Register'))

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo Accounts/input_Register Here_name'), 'TestUser')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo Accounts/input_Register Here_mobile'), '9978907812')

WebUI.setEncryptedText(findTestObject('Object Repository/Practo/Page_Practo Accounts/input_Register Here_password'), 'R2dZ4hvJ2ujoDGjQ2cClHw==')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo Accounts/input_Register Here_mobile'), '9978907812')

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/div_91(IND)65(SGP)63(PHL)60(MYS)62(IDN)55(B_2c5ff8'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/div_Full Name                              _fcd9f8'))

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo Accounts/input_Register Here_name'), 'Deepika Saini')

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo Accounts/input_Register Here_mobile'), '9978907812')

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/p_Mobile Number'))

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo Accounts/input_Register Here_mobile'), '9521731045')

WebUI.setEncryptedText(findTestObject('Object Repository/Practo/Page_Practo Accounts/input_Register Here_password'), 'R2dZ4hvJ2ujoDGjQ2cClHw==')

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/div_Join Practo                            _a77d58'))

WebUI.setEncryptedText(findTestObject('Object Repository/Practo/Page_Practo Accounts/input_Register Here_password'), 'c6bYgGNKPTHFFmXl0gERmg==')

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/button_Send OTP'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/img_Click verify once there are none left_rc-image-tile-33'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/img_Click verify once there are none left_r_1e3bc3'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/img_Click verify once there are none left_r_1e3bc3_1'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/button_Verify'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/div_Click verify once there are none left_r_f144f7'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/div_Click verify once there are none left_r_f144f7_1'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/img_Click verify once there are none left_r_1e3bc3_1_2'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/button_Verify'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/img_boats_rc-image-tile-33'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/img_boats_rc-image-tile-33_1'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/img_boats_rc-image-tile-33_1_2'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/img_boats_rc-image-tile-33_1_2_3'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/button_Verify'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/img_motorcycles_rc-image-tile-33'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/img_motorcycles_rc-image-tile-33_1'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/img_motorcycles_rc-image-tile-33_1_2'))

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/button_Verify'))

WebUI.setText(findTestObject('Object Repository/Practo/Page_Practo Accounts/input_OTP_mobile_token'), '383697')

WebUI.click(findTestObject('Object Repository/Practo/Page_Practo Accounts/button_Login'))

WebUI.closeBrowser()

